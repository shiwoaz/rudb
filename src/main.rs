mod common;
use common::{commands, print_prompt};

mod input_buffer;
use input_buffer::InputBuffer;

mod statement;

mod meta_command;

fn main() {
    let mut input_buffer = InputBuffer::new();
    loop {
        print_prompt();
        input_buffer.clear();
        input_buffer.read_command();
        commands(&input_buffer);
    }
}
