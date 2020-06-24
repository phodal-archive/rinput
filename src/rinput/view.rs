use std::sync::{Mutex, Arc};
use rustbox::{Color, RustBox, Style as RustBoxStyle};

use unicode_width::UnicodeWidthChar;

use crate::buffer::{Buffer};
use crate::mark::{Mark};

pub struct View {
    pub buffer: Arc<Mutex<Buffer>>,
    pub last_buffer: Option<Arc<Mutex<Buffer>>>,

    height: usize,
    width: usize,
    /// First character of the top line to be displayed
    top_line: Mark,

    /// Index into the top_line - used for horizontal scrolling
    left_col: usize,
}

impl View {
    pub fn new(buffer: Arc<Mutex<Buffer>>, width: usize, height: usize) -> View {
        let top_line = Mark::DisplayMark(0);

        View {
            buffer,
            last_buffer: None,
            height,
            width,
            top_line: top_line,
            left_col: 0,
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
