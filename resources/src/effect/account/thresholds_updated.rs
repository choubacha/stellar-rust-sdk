/// This effect can be the result of a set options operation and represents
/// the fact that an account's weight thresholds have changed.
#[derive(Debug, Deserialize)]
pub struct ThresholdsUpdated {
    account: String,
    low: u32,
    med: u32,
    high: u32,
}

impl ThresholdsUpdated {
    /// Creates a new ThresholdsUpdated effect
    pub fn new(account: String, low: u32, med: u32, high: u32) -> ThresholdsUpdated {
        ThresholdsUpdated {
            account,
            low,
            med,
            high,
        }
    }

    /// The public address of the account that had its thresholds updated.
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The sum weight for the low threshold.
    pub fn low(&self) -> u32 {
        self.low
    }

    /// The sum weight for the medium threshold.
    pub fn med(&self) -> u32 {
        self.med
    }

    /// The sum weight for the high threshold.
    pub fn high(&self) -> u32 {
        self.high
    }
}
