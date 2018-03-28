//! Contains effects that pertain to trades being executed.
use amount::Amount;
use asset::AssetIdentifier;

/// Enum representing all the different kinds of effects that represent
/// changes made to an account.
#[derive(Debug, Deserialize)]
pub enum Kind {
    /// An effect representing the fact that an trade occured
    Trade(Trade),
}

/// People on the Stellar network can make offers to buy or sell assets. When an offer is fully or
/// partially fulfilled, a trade happens.
#[derive(Debug, Deserialize)]
pub struct Trade {
    account: String,
    offer_id: i64,
    seller: String,
    sold_amount: Amount,
    sold_asset: AssetIdentifier,
    bought_amount: Amount,
    bought_asset: AssetIdentifier,
}

impl Trade {
    /// Creates a new Trade
    pub fn new(
        account: String,
        offer_id: i64,
        seller: String,
        sold_amount: Amount,
        sold_asset: AssetIdentifier,
        bought_amount: Amount,
        bought_asset: AssetIdentifier,
    ) -> Trade {
        Trade {
            account: account,
            offer_id: offer_id,
            seller: seller,
            sold_amount: sold_amount,
            sold_asset: sold_asset,
            bought_amount: bought_amount,
            bought_asset: bought_asset,
        }
    }

    /// The public address of the account that bought a trade
    pub fn account(&self) -> &String {
        &self.account
    }

    /// The id of the offer which was used in executing the trade
    pub fn offer_id(&self) -> i64 {
        self.offer_id
    }

    /// The public address of the other party in the trade
    pub fn seller(&self) -> &String {
        &self.seller
    }

    /// The amount of the sold asset that was exchanged in this trade
    pub fn sold_amount(&self) -> Amount {
        self.sold_amount
    }

    /// The asset being sold in the trade
    pub fn sold_asset(&self) -> &AssetIdentifier {
        &self.sold_asset
    }

    /// The amount of the bought asset that was exchanged in this trade
    pub fn bought_amount(&self) -> Amount {
        self.bought_amount
    }

    /// The asset being bought in the trade
    pub fn bought_asset(&self) -> &AssetIdentifier {
        &self.bought_asset
    }
}
