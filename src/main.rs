mod common;
use common::print_prompt;

mod input_buffer;
use input_buffer::InputBuffer;
use std::{io, process::exit};

pub fn commands(buffer: &InputBuffer) {
    let s = &buffer.buffer.trim()[..]; // remove new line char
    match s {
        _ if s == ".exit" => exit(0),
        un => println!("{} unrecognized command!", un),
    };
}

fn main() {
    let mut input_buffer = InputBuffer::new();
    loop {
        print_prompt();
        input_buffer.clear();
        input_buffer.read_command();
        commands(&input_buffer);
    }
}
