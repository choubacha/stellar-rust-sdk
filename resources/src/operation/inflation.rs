#[cfg(test)]
mod account_merge_tests {
    use serde_json;
    use operation::Operation;

    fn inflation_json() -> &'static str {
        include_str!("../../fixtures/operations/inflation.json")
    }

    #[test]
    fn it_parses_inflation_from_json() {
        let operation: Operation = serde_json::from_str(&inflation_json()).unwrap();
        assert!(operation.is_inflation());
        assert_eq!(operation.type_i(), 9);
    }
}
