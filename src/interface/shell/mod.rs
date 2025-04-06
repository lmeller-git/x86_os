use core::{
    fmt::Write,
    task::{Context, Poll, Waker},
};

use alloc::{boxed::Box, collections::BTreeMap, format, sync::Arc, vec::Vec};
use builtins::{cat, echo, exit, ls, mkdir, rm, touch, validate_checked};
use crossbeam_queue::ArrayQueue;
use parse::{Args, Cmd, Parser, Redirect};
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

pub trait OutStream: Write {
    fn write(&mut self, data: &str) {
        _ = self.write_str(data);
    }
}

impl Write for Redirect {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self {
            Self::Overwrite(p) => {
                let p = Path::new(p).canonicalize();
                _ = use_or_create(&p);
                fs::write(&p, s.bytes().collect()).unwrap();
            }
            Self::Append(p) => {
                let p = Path::new(p);
                let p = p.canonicalize();
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

#[allow(dead_code)]
struct Shell<'a> {
    fg: Option<Task>,
    bg: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
    instream: &'a mut ScancodeStream,
    pos: PathBuf,
}

#[allow(dead_code)]
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
        println!("{}", self.pos);
        print!("> ");

        let mut parser: Parser<VGAWriter> = Parser::new(&vga::WRITER);

        while let Some(arg) = parser.next(self.instream).await {
            self.execute(arg);
            self.run_ready_tasks().await;
            println!();
            println!("{}", self.pos);
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
        let current_pos: PathBuf = self.pos.clone();
        let out: Box<dyn OutStream> = if let Some(mut w) = cmd.out.clone() {
            if w.is_relative() {
                w.prepend(&format!("{}/", self.pos));
            }
            Box::new(w)
        } else {
            Box::new(Writer::new(&vga::WRITER))
        };
        let err: Box<dyn OutStream> = if let Some(mut w) = cmd.err.clone() {
            if w.is_relative() {
                w.prepend(&format!("{}/", self.pos));
            }
            Box::new(w)
        } else {
            Box::new(Writer::new(&vga::WRITER))
        };
        match cmd.cmd.as_str() {
            "cd" => self.cd(cmd.args.clone(), out, err),
            "ls" => self.spawn(Task::new(ls(current_pos, out, err, cmd.args.clone()))),
            "cat" => self.spawn(Task::new(cat(current_pos, out, err, cmd.args.clone()))),
            "mkdir" => self.spawn(Task::new(mkdir(current_pos, out, err, cmd.args.clone()))),
            "touch" => self.spawn(Task::new(touch(current_pos, out, err, cmd.args.clone()))),
            "echo" => self.spawn(Task::new(echo(out, err, cmd.args.clone()))),
            "exit" => self.spawn(Task::new(exit(out, err, cmd.args.clone()))),
            "rm" => self.spawn(Task::new(rm(current_pos, out, err, cmd.args.clone()))),
            "pwd" => self.pwd(cmd.args.clone(), out, err),
            _ => {}
        }
    }

    fn cd(
        &mut self,
        args: Args,
        mut _outstream: Box<dyn OutStream>,
        mut errstream: Box<dyn OutStream>,
    ) {
        let args: ArgParser<1> = match validate_checked(args, Vec::new()) {
            Ok(v) => v,
            Err(e) => {
                _ = writeln!(errstream, "wrong args to cd: {}", e);
                return;
            }
        };
        let p2 = Path::new(&args.positionals[0]);
        self.pos = if args.positionals[0].starts_with('/') {
            if !is_valid_dir(&p2) {
                _ = writeln!(errstream, "cannot move into {}", p2);
                self.pos.clone()
            } else {
                p2.canonicalize().into()
            }
        } else {
            let p = self.pos.as_path();
            let p = Path::try_concat(&p, &p2);
            if !is_valid_dir(&p) {
                _ = writeln!(errstream, "cannot move into {}", p);
                self.pos.clone()
            } else {
                p.canonicalize().into()
            }
        }
    }

    fn pwd(
        &self,
        _args: Args,
        mut outstream: Box<dyn OutStream>,
        mut _errstream: Box<dyn OutStream>,
    ) {
        _ = writeln!(outstream, "{}", self.pos);
    }
}

fn is_valid_dir(p: &Path) -> bool {
    match fs::get_node_info(p) {
        Ok(n) => n.is_dir,
        Err(_e) => false,
    }
}
