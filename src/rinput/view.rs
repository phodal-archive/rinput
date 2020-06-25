use std::sync::{Mutex, Arc};
use rustbox::{Color, RustBox, Style as RustBoxStyle};

use unicode_width::UnicodeWidthChar;

use crate::buffer::{Buffer};
use crate::buffer::Mark;
use crate::overlay::{Overlay, OverlayType, CommandPrompt};
use crate::textobject::{TextObject, Kind, Offset, Anchor};
use std::cmp;
use crate::utils;

pub struct View {
    pub buffer: Arc<Mutex<Buffer>>,
    pub last_buffer: Option<Arc<Mutex<Buffer>>>,
    pub overlay: Option<Box<Overlay>>,

    height: usize,
    width: usize,
    /// First character of the top line to be displayed
    top_line: Mark,

    /// The current View's cursor - a reference into the Buffer
    cursor: Mark,

    /// Index into the top_line - used for horizontal scrolling
    left_col: usize,

    /// Number of lines from the top/bottom of the View after which vertical
    /// scrolling begins.
    threshold: usize
}

impl View {
    pub fn new(buffer: Arc<Mutex<Buffer>>, width: usize, height: usize) -> View {
        let cursor = Mark::Cursor(0);
        let top_line = Mark::DisplayMark(0);

        {
            let mut b = buffer.lock().unwrap();

            b.set_mark(cursor, 0);
            b.set_mark(top_line, 0);
        }

        View {
            buffer,
            last_buffer: None,
            overlay: None,
            height,
            width,
            cursor,
            top_line: top_line,
            left_col: 0,
            threshold: 5,
        }
    }


    /// Get the height of the View.
    ///
    /// This is the height of the UIBuffer minus the status bar height.
    pub fn get_height(&self) -> usize {
        self.height - 1
    }

    /// Get the width of the View.
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn draw(&mut self, rb: &mut RustBox) {
        {
            let buffer = self.buffer.lock().unwrap();
            let height = self.get_height() - 1;
            let width = self.get_width() - 1;

            // FIXME: don't use unwrap here
            //        This will fail if for some reason the buffer doesnt have
            //        the top_line mark
            let mut lines = buffer.lines_from(self.top_line).unwrap().take(height);
            for y_position in 0..height {
                let line = lines.next().unwrap_or_else(Vec::new);
                draw_line(rb, &line, y_position, self.left_col);
            }
        }
    }

    /// Clear the buffer
    ///
    /// Fills every cell in the UIBuffer with the space (' ') char.
    pub fn clear(&mut self, rb: &mut RustBox) {
        for row in 0..self.height {
            for col in 0..self.width {
                rb.print_char(col, row, RustBoxStyle::empty(), Color::White, Color::Black, ' ');
            }
        }
    }

    /// Insert a chacter into the buffer & update cursor position accordingly.
    pub fn insert_char(&mut self, ch: char) {
        self.buffer.lock().unwrap().insert_char(self.cursor, ch as u8);
        // NOTE: the last param to char_width here may not be correct
        if let Some(ch_width) = utils::char_width(ch, false, 4, 1) {
            let obj = TextObject {
                kind: Kind::Char,
                offset: Offset::Forward(ch_width, Mark::Cursor(0))
            };
            self.move_mark(Mark::Cursor(0), obj)
        }
    }

    pub fn move_mark(&mut self, mark: Mark, object: TextObject) {
        self.buffer.lock().unwrap().set_mark_to_object(mark, object);
        self.maybe_move_screen();
    }

    pub fn set_overlay(&mut self, overlay_type: OverlayType) {
        match overlay_type {
            OverlayType::CommandPrompt => {
                self.overlay = Some(Box::new(CommandPrompt::new()));
            }
        }
    }

    // Delete chars from the first index of object to the last index of object
    pub fn delete_object(&mut self, object: TextObject) {
        self.buffer.lock().unwrap().remove_object(object);
    }

    pub fn delete_from_mark_to_object(&mut self, mark: Mark, object: TextObject) {
        let mut buffer = self.buffer.lock().unwrap();
        if let Some(mark_pos) = buffer.get_object_index(object) {
            if let Some(midx) = buffer.get_mark_idx(mark) {
                buffer.remove_from_mark_to_object(mark, object);
                buffer.set_mark(mark, cmp::min(mark_pos.absolute, midx));
            }
        }
    }

    /// Update the top_line mark if necessary to keep the cursor on the screen.
    fn maybe_move_screen(&mut self) {
        let mut buffer = self.buffer.lock().unwrap();
        if let (Some(cursor), Some((_, top_line))) = (buffer.get_mark_display_coords(self.cursor),
                                                      buffer.get_mark_display_coords(self.top_line)) {

            let width  = (self.get_width()  - self.threshold) as isize;
            let height = (self.get_height() - self.threshold) as isize;

            //left-right shifting
            self.left_col = match cursor.0 as isize - self.left_col as isize {
                x_offset if x_offset < self.threshold as isize => {
                    cmp::max(0, self.left_col as isize - (self.threshold as isize - x_offset)) as usize
                }
                x_offset if x_offset >= width => {
                    self.left_col + (x_offset - width + 1) as usize
                }
                _ => { self.left_col }
            };

            //up-down shifting
            match cursor.1 as isize - top_line as isize {
                y_offset if y_offset < self.threshold as isize && top_line > 0 => {
                    let amount = (self.threshold as isize - y_offset) as usize;
                    let obj = TextObject {
                        kind: Kind::Line(Anchor::Same),
                        offset: Offset::Backward(amount, self.top_line)
                    };
                    buffer.set_mark_to_object(self.top_line, obj);
                }
                y_offset if y_offset >= height => {
                    let amount = (y_offset - height + 1) as usize;
                    let obj = TextObject {
                        kind: Kind::Line(Anchor::Same),
                        offset: Offset::Forward(amount, self.top_line)
                    };
                    buffer.set_mark_to_object(self.top_line, obj);
                }
                _ => { }
            }
        }
    }
}

pub fn draw_line(rb: &mut RustBox, line: &[u8], idx: usize, left: usize) {
    let width = rb.width() - 1;
    let mut x = 0;

    for ch in line.iter().skip(left) {
        let ch = *ch as char;
        match ch {
            '\t' => {
                let w = 4 - x % 4;
                for _ in 0..w {
                    rb.print_char(x, idx, RustBoxStyle::empty(), Color::White, Color::Black, ' ');
                    x += 1;
                }
            }
            '\n' => {}
            _ => {
                rb.print_char(x, idx, RustBoxStyle::empty(), Color::White, Color::Black, ch);
                x += UnicodeWidthChar::width(ch).unwrap_or(1);
            }
        }
        if x >= width {
            break;
        }
    }

    // Replace any cells after end of line with ' '
    while x < width {
        rb.print_char(x, idx, RustBoxStyle::empty(), Color::White, Color::Black, ' ');
        x += 1;
    }

    // If the line is too long to fit on the screen, show an indicator
    let indicator = if line.len() > width + left { '→' } else { ' ' };
    rb.print_char(width, idx, RustBoxStyle::empty(), Color::White, Color::Black, indicator);
}
