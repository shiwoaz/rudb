#[derive(PartialEq, Debug)]
pub enum StatementType {
    StatementSelect,
    StatementInsert,
    Init,
}

pub struct Statement {
    pub statement_type: StatementType,
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
