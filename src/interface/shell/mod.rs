use core::{
    fmt::Write,
    task::{Context, Poll, Waker},
};

use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::String,
    sync::Arc,
    vec::{self, Vec},
};
use builtins::{cat, echo, exit, ls, mkdir, rm, touch};
use crossbeam_queue::ArrayQueue;
use futures_util::StreamExt;
use parse::{Args, Cmd, Parser, Redirect};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use utils::ArgParser;

use crate::{
    fs::{self, Path, PathBuf},
    io::{
        keyboard::ScancodeStream,
        vga::{self, VGAWriter},
        Writer, WriterBackend,
    },
    print, println,
    task::{executor::TaskWaker, Task, TaskId},
};

mod builtins;
mod parse;
mod repr;
mod utils;

struct OutWrapper<B>
where
    B: OutStream,
{
    inner: B,
}

impl<B> OutStream for OutWrapper<B> where B: OutStream {}

impl<B> Write for OutWrapper<B>
where
    B: OutStream,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.inner.write_str(s)
    }
}

#[derive(Default)]
pub struct TODO {}

pub trait OutStream: Write {
    fn write(&mut self, data: &str) {
        _ = self.write_str(data);
    }
}

impl Write for TODO {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl OutStream for TODO {}

impl Write for Redirect {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self {
            Self::Overwrite(p) => {
                let p = Path::new(p);
                _ = use_or_create(&p);
                fs::write(&p, s.bytes().collect()).unwrap();
            }
            Self::Append(p) => {
                let p = Path::new(p);
                _ = use_or_create(&p);
                let content = fs::read(&p).unwrap_or_default();
                let mut s: Vec<u8> = s.bytes().collect();
                s.splice(0..0, content);
                fs::write(&p, s).unwrap();
            }
        }
        Ok(())
    }
}

fn use_or_create(p: &Path) -> Result<(), fs::FSError> {
    match fs::get_node_info(p) {
        Err(fs::FSError::NodeNotFound(_)) => fs::insert_file(p),
        Err(e) => Err(e),
        Ok(info) => {
            if info.is_dir {
                fs::insert_file(p)
            } else {
                Ok(())
            }
        }
    }
}

impl OutStream for Redirect {}

impl<B> OutStream for Writer<B> where B: WriterBackend {}

pub async fn run_shell(scancodes: &mut ScancodeStream) {
    let mut shell = Shell::new(scancodes);
    shell.run().await
}

struct Shell<'a> {
    fg: Option<Task>,
    bg: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
    instream: &'a mut ScancodeStream,
    pos: PathBuf,
}

impl<'a> Shell<'a> {
    fn new(instream: &'a mut ScancodeStream) -> Self {
        Shell {
            fg: None,
            bg: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(10)),
            waker_cache: BTreeMap::new(),
            instream,
            pos: PathBuf::new("".into()),
        }
    }

    fn spawn(&mut self, task: Task) {
        let id = task.id;
        if self.bg.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(id).expect("queue full");
    }

    fn spawn_fg(&mut self, task: Task) {
        if self.fg.is_none() {
            self.fg = Some(task)
        }
    }

    async fn run(&mut self) {
        println!("Welcome to the shell");
        // get input, parse input, spawn tasks based on input, stop tasks on interrupt (ctrl-c), print task output
        print!("{}", self.pos);
        print!("> ");

        let mut parser: Parser<VGAWriter> = Parser::new(&vga::WRITER);

        while let Some(arg) = parser.next(self.instream).await {
            self.execute(arg);
            self.run_ready_tasks().await;
            print!("{}", self.pos);
            print!("> ");
        }
    }

    async fn run_ready_tasks(&mut self) {
        while let Some(task_id) = self.task_queue.pop() {
            let task = match self.bg.get_mut(&task_id) {
                Some(task) => task,
                None => continue,
            };
            let waker = self
                .waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::waker(task_id, self.task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    self.bg.remove(&task_id);
                    self.waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }

    fn execute(&mut self, cmd: &Cmd) {
        // println!("fine0");
        let current_pos: PathBuf = self.pos.clone();
        // println!("fine01");
        // TODO: get out and err from cmd and pass to cmds
        let out: Box<dyn OutStream> = if let Some(w) = cmd.out.clone() {
            Box::new(w)
        } else {
            Box::new(Writer::new(&vga::WRITER))
        };
        match cmd.cmd.as_str() {
            "cd" => self.cd(cmd.args.clone()),
            "ls" => self.spawn(Task::new(ls(current_pos, out, cmd.args.clone()))),
            "cat" => self.spawn(Task::new(cat(current_pos, out, cmd.args.clone()))),
            "mkdir" => self.spawn(Task::new(mkdir(current_pos, out, cmd.args.clone()))),
            "touch" => self.spawn(Task::new(touch(current_pos, out, cmd.args.clone()))),
            "echo" => self.spawn(Task::new(echo(out, cmd.args.clone()))),
            "exit" => self.spawn(Task::new(exit(out, cmd.args.clone()))),
            "rm" => self.spawn(Task::new(rm(current_pos, out, cmd.args.clone()))),
            _ => {}
        }
    }

    fn cd(&mut self, args: Args) {
        let args: ArgParser<1> = ArgParser::try_from(args)
            .unwrap()
            .validate(Vec::new())
            .unwrap();
        let p2 = Path::new(&args.positionals[0]);
        self.pos = if args.positionals[0].starts_with('/') {
            p2.into()
        } else {
            Path::try_concat(&self.pos.as_path(), &p2).into()
        }
    }
}
