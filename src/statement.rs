use std::error::Error;

use crate::parse_insert;

#[derive(PartialEq, Debug)]
pub enum StatementType {
    StatementSelect,
    StatementInsert,
    Init,
}

#[derive(Default, Debug, PartialEq)]
pub struct Row {
    id: u16,
    username: String,
    email: String,
}

impl Row {
    fn new(id: u16, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
        }
    }
}

pub struct Statement {
    pub statement_type: StatementType,
    pub row: Row,
}

impl Statement {
    pub fn new() -> Self {
        Self {
            statement_type: StatementType::Init,
            row: Row::default(),
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

pub fn prepare_statement(s: &str) -> Option<Row> {
    // parse_insert!(Row);
    let mut tokens = s.split(' ');
    tokens.next();
    let id = tokens.next().expect("no id field");
    let id = id.parse().expect("can't parse {id} to u16");
    let username = tokens.next().expect("no username field").to_string();
    let email = tokens.next().expect("no email field").to_string();
    Some(Row::new(id, username, email))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construct() {
        let mut p = Statement::new();
        p.construct_statement(StatementType::StatementSelect);
        assert_eq!(p.statement_type, StatementType::StatementSelect);
    }

    #[test]
    fn test_deal_with_statement() {
        let s = "select * from table";
        assert_eq!(deal_with_statement(s), Some(StatementType::StatementSelect));
        let s = "inserT * from table";
        assert_eq!(deal_with_statement(s), Some(StatementType::StatementInsert));
        let s = "other * from table";
        assert_eq!(deal_with_statement(s), None);
    }
}
