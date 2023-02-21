use std::io;

use crate::common::READ_LINE_ERR;

#[derive(Debug)]
pub struct InputBuffer {
    pub buffer: String,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn read_command(&mut self) {
        io::stdin()
            .read_line(&mut self.buffer)
            .expect(READ_LINE_ERR);
    }
}
