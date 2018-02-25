use std::fmt;

/// an amount
#[derive(Debug)]
pub struct Amount(i64);

/// We convert amounts to their stroop values (multiply by 10^7) on serialization.
/// When displaying values for end users, they are likely more interested
/// in the unit values, so we divide by 10^7
impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut raw_amount = self.0.to_string();
        let raw_digit_count = raw_amount.len();
        if raw_digit_count < 8 {
            let zero_pad = "0".repeat(7-raw_digit_count);
            return write!(f, "0.{}{}", zero_pad, raw_amount)
        } else {
            let period_index = raw_digit_count - 7;
            raw_amount.insert(period_index, '.');
            return write!(f, "{}", raw_amount)
        }
    }
}

#[cfg(test)]
mod amount_tests {
    use super::*;
    #[test]
    fn it_displays_small_numbers_as_float() {
        assert_eq!(format!("{}", Amount(10)), "0.0000010");
    }
    #[test]
    fn it_displays_large_numbers_as_float() {
        assert_eq!(format!("{}", Amount(123456789)), "12.3456789");
    }
}
