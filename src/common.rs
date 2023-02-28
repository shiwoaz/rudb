use std::io::{self, Write};

use crate::{
    input_buffer::InputBuffer,
    meta_command::{deal_with_meta_command, MetaResult},
    statement::{deal_with_statement, prepare_statement, Statement, StatementType},
};

pub fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

pub const READ_LINE_ERR: &'static str = "fail to read line";

pub fn commands(buffer: &InputBuffer) {
    let s = &buffer.buffer.trim()[..]; // remove new line char
    if s.chars().nth(0).unwrap_or('@') == '.' {
        match deal_with_meta_command(s) {
            Some(MetaResult::MetaCommandSuccess) => (),
            Some(MetaResult::MetaCommandUnrecognizedCommand) => {
                println!("{} unrecognized command!", s)
            }
            _ => (),
        };
    }
    let mut stm = Statement::new();
    match deal_with_statement(s) {
        Some(StatementType::StatementSelect) => {
            println!("select got {}", s);
            stm.construct_statement(StatementType::StatementSelect);
        }
        Some(StatementType::StatementInsert) => {
            println!("insert got {}", s);
            stm.construct_statement(StatementType::StatementInsert);
            if let Some(row) = prepare_statement(s) {
                println!("{:?}", row);
                stm.row = row;
            } // parse error will painc for now
        }
        _ => println!("{} unrecognized command!", s),
    }

    execute(stm);
}

fn execute(statement: Statement) {
    match statement.statement_type {
        StatementType::Init => (),
        StatementType::StatementInsert => println!("execute insert"),
        StatementType::StatementSelect => println!("execute select"),
    }
}
