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

    /// Handle key events
///
/// Key events can be handled in an Overlay, OR in the current Mode.
///
/// If there is an active Overlay, the key event is sent there, which gives
/// back an OverlayEvent. We then parse this OverlayEvent and determine if
/// the Overlay is finished and can be cleared. The response from the
/// Overlay is then converted to a Command and sent off to be handled.
///
/// If there is no active Overlay, the key event is sent to the current
/// Mode, which returns a Command which we dispatch to handle_command.
    fn handle_key_event(&mut self, event: Event) {
        let key = Key::from_event(&mut self.rb, event);

        let key = match key {
            Some(k) => k,
            None => return
        };

        // println!("{}", key);
        self.running = false
    }

    pub fn start(&mut self) {
        while self.running {
            self.draw();
            self.rb.present();

            match self.rb.poll_event(true) {
                Ok(Event::ResizeEvent(width, height)) => {

                },
                Ok(key_event) => self.handle_key_event(key_event),
                _ => {}
            }
        }
    }
}