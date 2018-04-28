#![macro_use]
macro_rules! append_to_buffer {
    ($buffer:expr, $($args:tt)*) => {
        $buffer.push_str(&format!($($args)*));
        $buffer.push_str("\n");
    }
}
pub struct Simple;

mod account;
mod asset;
mod effect;
mod ledger;
mod trade_aggregation;
mod transaction;

#[cfg(test)]
mod tests {

    #[test]
    fn it_appends_to_a_buffer() {
        let mut buffer = String::new();
        append_to_buffer!(buffer, "Fantastic Mr Fox");

        assert_eq!(buffer, "Fantastic Mr Fox\n".to_string());
    }
}
