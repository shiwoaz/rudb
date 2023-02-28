#[macro_export]
macro_rules! parse_insert {
    ($ins: ty, $s:expr) => {{
        let mut tokens = $s.split(' ');
        println!("{:?}", tokens.next()); // insert
        let id = tokens.next();
        let username = tokens.next();
        let email = tokens.next();
        // <$ins>::new(id, username, email)
        <$ins>::default()
    }};
}

#[cfg(test)]
mod test {
    use crate::statement::Row;

    #[test]
    fn test_parse_insert() {
        let row = Row::default();
        let insert_sql = "insert 1 cstack foo@bar.com";
        assert_eq!(row, parse_insert!(Row, insert_sql));
    }
}
