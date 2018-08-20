use std::str::FromStr;
use stellar_client::endpoint::trade::SegmentResolution;

/// Wrapper around the segment resolution.
#[derive(Debug, Eq, PartialEq)]
pub struct Resolution(SegmentResolution);

impl Resolution {
    pub fn inner(&self) -> SegmentResolution {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseResolutionError {
    InvalidResolution,
}

impl FromStr for Resolution {
    type Err = ParseResolutionError;

    fn from_str(s: &str) -> Result<Resolution, ParseResolutionError> {
        match s {
            "1m" => Ok(Resolution(SegmentResolution::OneMin)),
            "5m" => Ok(Resolution(SegmentResolution::FiveMin)),
            "15m" => Ok(Resolution(SegmentResolution::FifteenMin)),
            "1h" => Ok(Resolution(SegmentResolution::OneHour)),
            "1d" => Ok(Resolution(SegmentResolution::OneDay)),
            "1w" => Ok(Resolution(SegmentResolution::OneWeek)),
            _ => return Err(ParseResolutionError::InvalidResolution),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_into_all_valid_resolutions() {
        assert_eq!("1m".parse(), Ok(Resolution(SegmentResolution::OneMin)));
        assert_eq!("5m".parse(), Ok(Resolution(SegmentResolution::FiveMin)));
        assert_eq!("15m".parse(), Ok(Resolution(SegmentResolution::FifteenMin)));
        assert_eq!("1h".parse(), Ok(Resolution(SegmentResolution::OneHour)));
        assert_eq!("1d".parse(), Ok(Resolution(SegmentResolution::OneDay)));
        assert_eq!("1w".parse(), Ok(Resolution(SegmentResolution::OneWeek)));
    }

    #[test]
    fn errs_when_not_valid() {
        assert_eq!(
            "1y".parse::<Resolution>(),
            Err(ParseResolutionError::InvalidResolution)
        );
    }
}
