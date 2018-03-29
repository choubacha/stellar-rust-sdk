use chrono::prelude::*;
use amount::Amount;
use deserialize;

/// Transactions are the basic unit of change in the Stellar Network.
/// A transaction is a grouping of operations.
///
/// To learn more about the concept of transactions in the Stellar network, take a look at the Stellar transactions concept guide.
#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    id: String,
    paging_token: String,
    hash: String,
    ledger: u32,
    created_at: DateTime<Utc>,
    source_account: String,
    #[serde(deserialize_with = "deserialize::from_str")]
    source_account_sequence: u64,
    fee_paid: i64,
    operation_count: u32,
    envelope_xdr: String,
    result_xdr: String,
    result_meta_xdr: String,
    fee_meta_xdr: String,
}

impl Transaction {
    /// The canonical id of this transaction, suitable for use as the :id parameter for url
    /// templates that require a transaction’s ID.
    pub fn id(&self) -> &String {
        &self.id
    }

    /// A paging token suitable for use as the cursor parameter to transaction collection
    /// resources.
    pub fn paging_token(&self) -> &String {
        &self.paging_token
    }

    /// A hex-encoded SHA-256 hash of the transaction’s XDR-encoded form.
    pub fn hash(&self) -> &String {
        &self.hash
    }

    /// Sequence number of the ledger in which this transaction was applied.
    pub fn ledger(&self) -> u32 {
        self.ledger
    }

    /// The time the transaction was processed.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// The account that signed and initiated the transaction
    pub fn source_account(&self) -> &String {
        &self.source_account
    }

    /// The current sequence number that can be used when submitting a transaction
    /// from the transaction signing account
    pub fn source_account_sequence(&self) -> u64 {
        self.source_account_sequence
    }

    /// The fee paid by the source account of this transaction when the transaction was applied to
    /// the ledger.
    pub fn fee_paid(&self) -> i64 {
        self.fee_paid
    }

    /// The fee represented as an amount.  This is useful when subtracting fees from
    /// other amounts in your model
    pub fn fee_as_amount(&self) -> Amount {
        Amount::new(self.fee_paid)
    }

    /// The number of operations that are contained within this transaction.
    pub fn operation_count(&self) -> u32 {
        self.operation_count
    }

    /// A base64 encoded string of the raw TransactionEnvelope xdr struct for this transaction
    pub fn envelope_xdr(&self) -> &String {
        &self.envelope_xdr
    }

    /// A base64 encoded string of the raw TransactionResultPair xdr struct for this transaction
    pub fn result_xdr(&self) -> &String {
        &self.result_xdr
    }

    /// A base64 encoded string of the raw TransactionMeta xdr struct for this transaction
    pub fn result_meta_xdr(&self) -> &String {
        &self.result_meta_xdr
    }

    /// A base64 encoded string of the raw LedgerEntryChanges xdr struct produced by taking fees
    /// for this transaction.
    pub fn fee_meta_xdr(&self) -> &String {
        &self.fee_meta_xdr
    }
}

#[cfg(test)]
mod transaction_tests {
    use super::*;
    use serde_json;

    fn transaction_json() -> &'static str {
        include_str!("../fixtures/transaction.json")
    }

    #[test]
    fn it_parses_into_a_transaction() {
        let transaction: Transaction = serde_json::from_str(&transaction_json()).unwrap();
        assert_eq!(
            transaction.id(),
            "648da0d47aa3b3b20afd4499a68f89b6d10ead8b1f38858e99b1d94b6fef6e69"
        );
        assert_eq!(transaction.paging_token(), "71946212651044864");
        assert_eq!(
            transaction.hash(),
            "648da0d47aa3b3b20afd4499a68f89b6d10ead8b1f38858e99b1d94b6fef6e69"
        );
        assert_eq!(transaction.ledger(), 16751283);
        assert_eq!(
            transaction.created_at(),
            Utc.ymd(2018, 3, 10).and_hms(23, 16, 42)
        );
        assert_eq!(
            transaction.source_account(),
            "GB6YPGW5JFMMP2QB2USQ33EUWTXVL4ZT5ITUNCY3YKVWOJPP57CANOF3"
        );
        assert_eq!(transaction.source_account_sequence(), 2394452857640034);
        assert_eq!(transaction.fee_paid(), 100);
        assert_eq!(transaction.fee_as_amount(), Amount::new(100));
        assert_eq!(transaction.operation_count(), 1);
        assert_eq!(
            transaction.envelope_xdr(),
            "AAAAAH2Hmt1JWMfqAdUlDeyUtO9V8zPqJ0aLG8KrZyXv78QGAAAAZAAIgb4AAtRiAAAAAAAAAAEAAAAAAAAA\
             AQAAAAAAAAABAAAAAJZgy/0KAk+3JQwG8hPGBNTZVGew2Joi1TwkVBdwPn9QAAAAAAAAAAA7mUNgAAAAAAAAA\
             AHv78QGAAAAQITCXzWfgHgAjF3djx1VK9JK08UypfpftzFoyNXv7A0Agau/ur/3/+ZZtQb8xSsao8yVAsTiV4\
             ttiT/HqfvvlAk="
        );
        assert_eq!(
            transaction.result_xdr(),
            "AAAAAAAAAGQAAAAAAAAAAQAAAAAAAAABAAAAAAAAAAA="
        );
        assert_eq!(
            transaction.result_meta_xdr(),
            "AAAAAAAAAAEAAAAEAAAAAwD/mrMAAAAAAAAAAH2Hmt1JWMfqAdUlDeyUtO9V8zPqJ0aLG8KrZyXv78QGACqz0\
             JcAUz4ACIG+AALUYgAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAQD/mrMAAAAAAAAAAH2Hmt1\
             JWMfqAdUlDeyUtO9V8zPqJ0aLG8KrZyXv78QGACqz0FtnD94ACIG+AALUYgAAAAAAAAAAAAAAAAAAAAABAAAAA\
             AAAAAAAAAAAAAAAAAAAAwD45pUAAAAAAAAAAJZgy/0KAk+3JQwG8hPGBNTZVGew2Joi1TwkVBdwPn9QAAAAAAD\
             ccSUA7xgIAAAAAQAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAQD/mrMAAAAAAAAAAJZgy/0KA\
             k+3JQwG8hPGBNTZVGew2Joi1TwkVBdwPn9QAAAAADx1tIUA7xgIAAAAAQAAAAAAAAAAAAAAAAAAAAABAAAAAAA\
             AAAAAAAAAAAAA"
        );
        assert_eq!(
            transaction.fee_meta_xdr(),
            "AAAAAgAAAAMA/5qyAAAAAAAAAAB9h5rdSVjH6gHVJQ3slLTvVfMz6idGixvCq2cl7+/EBgAqs9CXAFOiAAiBv\
             gAC1GEAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAEA/5qzAAAAAAAAAAB9h5rdSVjH6gHVJQ3\
             slLTvVfMz6idGixvCq2cl7+/EBgAqs9CXAFM+AAiBvgAC1GIAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAA\
             AAAAA=="
        )
    }
}
