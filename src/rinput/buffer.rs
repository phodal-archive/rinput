use std::cmp;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Stdin, Read};
use std::collections::HashMap;

use gapbuffer::GapBuffer;

use crate::input::Input;
use crate::mark::Mark;
use crate::iterators::Lines;


#[derive(PartialEq, Debug)]
pub struct MarkPosition {
    pub absolute: usize,
    absolute_line_start: usize,
    line_number: usize,
}

impl MarkPosition {
    fn start() -> MarkPosition {
        MarkPosition {
            absolute: 0,
            line_number: 0,
            absolute_line_start: 0,
        }
    }
}

pub struct Buffer {
    /// Current buffers text
    text: GapBuffer<u8>,

    /// Table of marked indices in the text
    marks: HashMap<Mark, MarkPosition>,

    pub file_path: Option<PathBuf>,
}

impl Buffer {
    /// Constructor for empty buffer.
    pub fn new() -> Buffer {
        Buffer {
            text: GapBuffer::new(),
            marks: HashMap::new(),
            file_path: None,
        }
    }

    /// Length of the text stored in this buffer.
    pub fn len(&self) -> usize {
        self.text.len() + 1
    }

    /// Sets the mark to a given absolute index. Adds a new mark or overwrites an existing mark.
    pub fn set_mark(&mut self, mark: Mark, idx: usize) {
        if let Some(mark_pos) = get_line_info(idx, &self.text) {
            if let Some(existing_pos) = self.marks.get_mut(&mark) {
                existing_pos.absolute = mark_pos.absolute;
                existing_pos.line_number = mark_pos.line_number;
                existing_pos.absolute_line_start = mark_pos.absolute_line_start;
                return;
            }
            self.marks.insert(mark, mark_pos);
        }
    }

    /// Creates an iterator on the text by lines that begins at the specified mark.
    pub fn lines_from(&self, mark: Mark) -> Option<Lines> {
        if let Some(mark_pos) = self.marks.get(&mark) {
            if mark_pos.absolute < self.len() {
                return Some(Lines {
                    buffer: &self.text,
                    tail: mark_pos.absolute,
                    head: self.len(),
                })
            }
        }

        None
    }
}

fn get_line_info(mark: usize, text: &GapBuffer<u8>) -> Option<MarkPosition> {
    let val = cmp::min(mark, text.len());
    let line_starts: Vec<usize> = (0..val + 1).rev().filter(|idx| *idx == 0 || text[*idx - 1] == b'\n').collect();


    if line_starts.is_empty() {
        None
    } else {
        let mut mark_pos = MarkPosition::start();
        mark_pos.absolute_line_start = line_starts[0];
        mark_pos.line_number = line_starts.len() - 1;
        mark_pos.absolute = mark;
        Some(mark_pos)
    }
}

// This is a bit of a hack to get around an error I was getting when
// implementing From<R: Read> for Buffer with From<PathBuf> for Buffer.
// The compiler was telling me this was a conflicting implementation even
// though Read is not implemented for PathBuf. Changing R: Read to
// R: Read + BufferFrom fixes the error.
//
// TODO: investigate this further - possible compiler bug?
pub trait BufferFrom {}
impl BufferFrom for Stdin {}
impl BufferFrom for File {}

impl From<PathBuf> for Buffer {
    fn from(path: PathBuf) -> Buffer {
        match File::open(&path) {
            Ok(file) => {
                let mut buf = Buffer::from(file);
                buf.file_path = Some(path);
                buf
            }
            Err(_) => {
                Buffer::new()
            }
        }
    }
}

impl<R: Read + BufferFrom> From<R> for Buffer {
    fn from(mut reader: R) -> Buffer {
        let mut buff = Buffer::new();
        let mut contents = String::new();
        if reader.read_to_string(&mut contents).is_ok() {
            buff.text.extend(contents.bytes());
        }
        buff
    }
}

impl From<Input> for Buffer {
    fn from(input: Input) -> Buffer {
        match input {
            Input::Filename(path) => {
                match path {
                    Some(path) => Buffer::from(PathBuf::from(path)),
                    None       => Buffer::new(),
                }
            },
            Input::Stdin(reader) => {
                Buffer::from(reader)
            }
        }
    }
}

