macro_rules! query {
    (from $range_variable:ident; in $data_source:ident; select $select_expr:expr;) =>
        { $data_source.map(|$range_variable| $select_expr) };
    (from $range_variable:ident; in $data_source:ident; $(where $where_expr:expr;)* select $select_expr:expr;) =>
        { $data_source.filter(|&$range_variable| (true $(&& $where_expr)*) ).map(|$range_variable| $select_expr) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn select_query() {
        let x = 1..100;
        let y: Vec<_> = x.clone().map(|i| i * 2).collect();
        let z: Vec<_> = query!(from i; in x; select i * 2;).collect();
        assert_eq!(z, y);
    }

    #[test]
    fn where_query() {
        let x = 1..100;
        let y: Vec<_> = x.clone().filter(|&i| i % 2 == 0).map(|i| i * 2).collect();
        let z: Vec<_> = query!(from i; in x; where i % 2 == 0; select i * 2;).collect();
        assert_eq!(z, y);
    }

    #[test]
    fn multiple_where_query() {
        let x = 1..100;
        let y: Vec<_> = x.clone().filter(|&i| i % 2 == 0 && i % 4 == 0).map(|i| i * 2).collect();
        let z: Vec<_> = query!(from i; in x; where i % 2 == 0; where i % 4 == 0; select i * 2;).collect();
        assert_eq!(z, y);
    }

    #[test]
    fn multiple_conditions_in_where_query() {
        let x = 1..100;
        let y: Vec<_> = x.clone().filter(|&i| i % 2 == 0 && i % 4 == 0).map(|i| i * 2).collect();
        let z: Vec<_> = query!(from i; in x; where i % 2 == 0 && i % 4 == 0; select i * 2;).collect();
        assert_eq!(z, y);
    }
}
