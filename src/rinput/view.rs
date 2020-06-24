use std::sync::{Mutex, Arc};

use crate::buffer::{Buffer};

pub struct View {
    pub buffer: Arc<Mutex<Buffer>>,
    pub last_buffer: Option<Arc<Mutex<Buffer>>>,

    height: usize,
    width: usize,
}

impl View {
    pub fn new(buffer: Arc<Mutex<Buffer>>, width: usize, height: usize) -> View {
        View {
            buffer,
            last_buffer: None,
            height,
            width,
        }
    }
}