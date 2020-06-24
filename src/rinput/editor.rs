use std::path::PathBuf;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Mutex, Arc};
use std::sync::mpsc::channel;
use std::str;

use rustbox::{RustBox, Event};

use crate::input::Input;
use crate::keyboard::Key;
use crate::buffer::Buffer;
use crate::command::Command;
use crate::view::View;

pub struct Editor {
    buffers: Vec<Arc<Mutex<Buffer>>>,
    view: View,
    rb: RustBox,
    running: bool,

    command_queue: Receiver<Command>,
    command_sender: Sender<Command>,
}

impl Editor {
    pub fn new(source: Input, rb: RustBox) -> Editor {
        let height = rb.height();
        let width = rb.width();

        let (snd, recv) = channel();

        let mut buffers = Vec::new();

        let buffer = match source {
            Input::Filename(path) => {
                match path {
                    Some(path) => Buffer::from(PathBuf::from(path)),
                    None => Buffer::new(),
                }
            }
            Input::Stdin(reader) => {
                Buffer::from(reader)
            }
        };

        buffers.push(Arc::new(Mutex::new(buffer)));

        println!("............");
        let view = View::new(buffers[0].clone(), width, height);

        Editor {
            rb,
            buffers,
            view,
            running: true,

            command_queue: recv,
            command_sender: snd,
        }
    }

    /// Draw the current view to the frontend
    fn draw(&mut self) {
        self.view.draw(&mut self.rb);
    }

    pub fn start(&mut self) {
        while self.running {
            self.draw();
            self.rb.present();
        }
    }
}