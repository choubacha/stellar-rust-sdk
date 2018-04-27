use std::str::FromStr;
use self::Resolution::{Day, Hour, Min, Sec};

/// Represents durations of time for the trade aggregations endpoint.
pub enum Resolution {
    Sec(u32),
    Min(u32),
    Hour(u32),
    Day(u32),
}

#[derive(Debug)]
pub enum ParseResolutionError {
    InvalidNumber,
    InvalidUnit,
}

impl Resolution {
    /// Convert resolutions to ms to represent time intervals in the format requred by the horizon
    /// API
    pub fn to_ms(&self) -> u64 {
        match *self {
            Sec(sec) => u64::from(sec) * 1_000,
            Min(mins) => u64::from(mins) * Sec(60).to_ms(),
            Hour(hours) => u64::from(hours) * Min(60).to_ms(),
            Day(days) => u64::from(days) * Hour(24).to_ms(),
        }
    }
}

impl FromStr for Resolution {
    type Err = ParseResolutionError;

    fn from_str(s: &str) -> Result<Resolution, ParseResolutionError> {
        let (digit, unit) = s.split_at(s.len() - 1);
        let num = digit
            .parse()
            .map_err(|_| ParseResolutionError::InvalidNumber)?;
        Ok(match unit {
            "s" => Sec(num),
            "m" => Min(num),
            "h" => Hour(num),
            "d" => Day(num),
            _ => return Err(ParseResolutionError::InvalidUnit),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_resolution_to_ms(s: &str) -> u64 {
        let res: Resolution = s.parse().unwrap();
        res.to_ms()
    }

    #[test]
    fn it_can_parse_and_convert_resolutions_to_ms() {
        assert_eq!(parse_resolution_to_ms("25s"), 25_000);
        assert_eq!(parse_resolution_to_ms("25m"), 1_500_000);
        assert_eq!(parse_resolution_to_ms("25h"), 90_000_000);
        assert_eq!(parse_resolution_to_ms("25d"), 2_160_000_000);
    }
}
