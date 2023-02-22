use std::process::Command;

pub enum StatementType {
    StatementSelect,
    StatementInsert,
    Init,
}

pub struct Statement {
    statement_type: StatementType,
}

impl Statement {
    pub fn new() -> Self {
        Self {
            statement_type: StatementType::Init,
        }
    }

    pub fn construct_statement(&mut self, x: StatementType) {
        self.statement_type = x;
    }
}

pub fn deal_with_statement(s: &str) -> Option<StatementType> {
    let binding = s.to_lowercase();
    let mut command = binding.trim().split_whitespace();
    if let Some(cmd) = command.next() {
        match cmd {
            "select" => Some(StatementType::StatementSelect),
            "insert" => Some(StatementType::StatementInsert),
            _ => None,
        }
    } else {
        None
    }
}
