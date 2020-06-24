use std::path::PathBuf;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Mutex, Arc};
use std::sync::mpsc::channel;
use std::str;
use std::collections::HashMap;

use rustbox::{RustBox, Event};

use crate::input::Input;
use crate::keyboard::Key;
use crate::buffer::Buffer;
use crate::command::{Command, BuilderArgs, BuilderEvent, Action, Instruction, Operation};
use crate::view::View;
use crate::modes::{Mode, StandardMode};


type EditorCommand = fn(Option<BuilderArgs>) -> Command;
lazy_static! {
    pub static ref ALL_COMMANDS: HashMap<&'static str, EditorCommand> = {
        let mut map: HashMap<&'static str, EditorCommand> = HashMap::new();

        map.insert("editor::quit", Command::exit_editor);
        map.insert("editor::save_buffer", Command::save_buffer);
        map.insert("editor::noop", Command::noop);

        map.insert("editor::undo", Command::undo);
        map.insert("editor::redo", Command::redo);
        map.insert("editor::set_mode", Command::set_mode);

        map.insert("editor::set_overlay", Command::set_overlay);

        map.insert("buffer::move_cursor", Command::move_cursor);
        map.insert("buffer::insert_char", Command::insert_char);
        map.insert("buffer::insert_tab", Command::insert_tab);
        map.insert("buffer::delete_char", Command::delete_char);


        map
    };
}

pub struct Editor {
    buffers: Vec<Arc<Mutex<Buffer>>>,
    view: View,
    rb: RustBox,
    mode: Box<dyn Mode>,

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

        let mode = Box::new(StandardMode::new());

        Editor {
            rb,
            buffers,
            view,
            running: true,
            mode,
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

        let command = match self.view.overlay {
            None => self.mode.handle_key_event(key),
            Some(ref mut overlay) => overlay.handle_key_event(key),
        };

        if let BuilderEvent::Complete(c) = command {
            self.view.overlay = None;
            self.view.clear(&mut self.rb);

            match ALL_COMMANDS.get(&*c.command_name) {
                Some(cmd) => {
                    let cmd = cmd(c.args);
                    let _ = self.command_sender.send(cmd);
                }
                None => {
                    panic!("Unknown command: {}", c.command_name);
                }
            }

            // let _ = self.command_sender.send(c);
        }

    }

    /// Handle the given command, performing the associated action
    fn handle_command(&mut self, command: Command) {
        let repeat = if command.number > 0 {
            command.number
        } else { 1 };
        for _ in 0..repeat {
            match command.action {
                Action::Instruction(_) => self.handle_instruction(command.clone()),
                Action::Operation(_) => self.handle_operation(command.clone()),
            }
        }
    }

    fn handle_instruction(&mut self, command: Command) {
        match command.action {
            Action::Instruction(Instruction::ExitEditor) => {
                self.running = false;
            }

            _ => {}
        }
    }

    fn handle_operation(&mut self, command: Command) {
        match command.action {
            Action::Operation(Operation::Insert(c)) => {
                for _ in 0..command.number {
                    self.view.insert_char(c)
                }
            }
            Action::Operation(Operation::DeleteObject) => {
                if let Some(obj) = command.object {
                    self.view.delete_object(obj);
                }
            }
            Action::Operation(Operation::DeleteFromMark(m)) => {
                if command.object.is_some() {
                    self.view.delete_from_mark_to_object(m, command.object.unwrap())
                }
            }

            Action::Instruction(_) => {}
            _ => {}
        }
    }

    pub fn start(&mut self) {
        while self.running {
            self.draw();
            self.rb.present();

            match self.rb.poll_event(true) {
                Ok(Event::ResizeEvent(width, height)) => {}
                Ok(key_event) => self.handle_key_event(key_event),
                _ => {}
            }

            while let Ok(message) = self.command_queue.try_recv() {
                self.handle_command(message)
            }
        }
    }
}