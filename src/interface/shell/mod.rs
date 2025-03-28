use core::task::Waker;

use alloc::{collections::BTreeMap, sync::Arc};
use builtins::print_stuff;
use crossbeam_queue::ArrayQueue;
use futures_util::StreamExt;
use parse::Parser;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

use crate::{
    interface::exit,
    io::keyboard::ScancodeStream,
    print, println,
    task::{Task, TaskId},
};

mod builtins;
mod parse;
mod repr;
mod utils;

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
}

impl<'a> Shell<'a> {
    fn new(instream: &'a mut ScancodeStream) -> Self {
        Shell {
            fg: None,
            bg: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(10)),
            waker_cache: BTreeMap::new(),
            instream,
        }
    }

    fn spawn(&mut self, task: Task) {}

    async fn run(&mut self) {
        println!("Welcome to the shell");
        // get input, parse input, spawn tasks based on input, stop tasks on interrupt (ctrl-c), print task output(task does it themselves)
        print!("> ");

        let mut parser = Parser::new();

        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let mut i = 0;
        // let mut scancodes = ScancodeStream::new();
        let mut keyboard = Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore,
        );

        while let Some(scancode) = self.instream.next().await {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::Unicode(character) => {
                            if character == '\n' {
                                exit(&buf);
                                i = 0;
                                print!("\n> ");
                            } else if i < 4 {
                                buf[i] = character as u8;
                                i += 1;
                                print!("{}", character);
                            } else {
                                i = 0;
                                buf[i] = character as u8;
                                print!("{}", character);
                            }
                        }
                        DecodedKey::RawKey(key) => print!("{:?}", key),
                    }
                }
            }
        }
    }
}
