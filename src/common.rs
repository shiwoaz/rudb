use std::{
    io::{self, Write},
    process::exit,
};

use crate::input_buffer::InputBuffer;

pub fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

pub const READ_LINE_ERR: &'static str = "fail to read line";

pub fn commands(buffer: &InputBuffer) {
    let s = &buffer.buffer.trim()[..]; // remove new line char
    match s {
        _ if s == ".exit" => exit(0),
        un => println!("{} unrecognized command!", un),
    };
}
