use chrono::prelude::*;
use amount::Amount;

/// A ledger represents the state of the Stellar universe at a given point in time. It contains the list of all the accounts and balances, all the orders in the distributed exchange, and any other data that persists.
/// The first ledger in the history of the network is called the genesis ledger.
#[derive(Debug, Deserialize)]
pub struct Ledger {
    id: String,
    paging_token: String,
    hash: String,
    sequence: u32,
    transaction_count: i64,
    operation_count: i64,
    closed_at: DateTime<Utc>,
    total_coins: Amount,
    fee_pool: Amount,
    base_fee_in_stroops: i64,
    base_reserve_in_stroops: i64,
    max_tx_set_size: u32,
    protocol_version: u32,
}

impl Ledger {
    /// The unique identifier for this ledger
    pub fn id(&self) -> &String {
        &self.id
    }
    /// A paging token suitable for use as the cursor parameter to ledger collection
    /// resources.
    pub fn paging_token(&self) -> &String {
        &self.paging_token
    }

    /// A hex-encoded SHA-256 hash of the ledger’s XDR-encoded form.
    pub fn hash(&self) -> &String {
        &self.hash
    }

    /// Sequence number of this ledger, suitable for use as the as the :id parameter for url templates that require a ledger number.
    pub fn sequence(&self) -> u32 {
        self.sequence
    }

    /// The number of transactions in this ledger.
    pub fn transaction_count(&self) -> i64 {
        self.transaction_count
    }

    /// The number of operations in this ledger.
    pub fn operation_count(&self) -> i64 {
        self.operation_count
    }

    /// An ISO 8601 formatted string of when this ledger was closed.
    pub fn closed_at(&self) -> DateTime<Utc> {
        self.closed_at
    }

    /// The total number of lumens in circulation.
    pub fn total_coins(&self) -> Amount {
        self.total_coins
    }

    /// The sum of all transaction fees (in lumens) since the last inflation operation. They are redistributed during inflation.
    pub fn fee_pool(&self) -> Amount {
        self.fee_pool
    }

    /// The fee the network charges per operation in a transaction.
    pub fn base_fee_in_stroops(&self) -> i64 {
        self.base_fee_in_stroops
    }

    /// The fee the network charges per operation in a transaction as an amount asset.
    pub fn base_fee_as_amount(&self) -> Amount {
        Amount::new(self.base_fee_in_stroops)
    }

    /// The reserve the network uses when calculating an account’s minimum balance.
    pub fn base_reserve_in_stroops(&self) -> i64 {
        self.base_reserve_in_stroops
    }

    /// The reserve the network uses when calculating an account’s minimum balance as an amount asset.
    pub fn base_reserve_as_amount(&self) -> Amount {
        Amount::new(self.base_reserve_in_stroops)
    }

    /// The maximum number of transactions validators have agreed to process in a given ledger.
    pub fn max_tx_set_size(&self) -> u32 {
        self.max_tx_set_size
    }

    /// The protocol version that the stellar network was running when this ledger was committed.
    pub fn protocol_version(&self) -> u32 {
        self.protocol_version
    }
}

#[cfg(test)]
mod ledger_tests {
    use super::*;
    use serde_json;

    fn ledger_json() -> &'static str {
        include_str!("../fixtures/ledger.json")
    }

    #[test]
    fn it_parses_into_a_transaction() {
        let ledger: Ledger = serde_json::from_str(&ledger_json()).unwrap();
        assert_eq!(
            ledger.id(),
            "eee9e6e02899365ecae4c37e52db7d99e2d130baf4ec1856d311bb546df1d0ad"
        );
        assert_eq!(ledger.paging_token(), "300042120331264");
        assert_eq!(
            ledger.hash(),
            "eee9e6e02899365ecae4c37e52db7d99e2d130baf4ec1856d311bb546df1d0ad"
        );
        assert_eq!(ledger.sequence(), 69859);
        assert_eq!(ledger.transaction_count(), 0);
        assert_eq!(ledger.operation_count(), 0);
        assert_eq!(ledger.closed_at(), Utc.ymd(2017, 3, 23).and_hms(20, 13, 23));
        assert_eq!(ledger.total_coins(), Amount::new(1_000_000_000_000_000_000));
        assert_eq!(ledger.fee_pool(), Amount::new(18_000_080_200));
        assert_eq!(ledger.base_fee_in_stroops(), 100);
        assert_eq!(ledger.base_reserve_in_stroops(), 100000000);
        assert_eq!(ledger.max_tx_set_size(), 50);
        assert_eq!(ledger.protocol_version(), 4);
    }
}
