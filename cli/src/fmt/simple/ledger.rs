use super::Simple;
use fmt::Render;
use stellar_client::resources::Ledger;

impl Render<Ledger> for Simple {
    fn render(&self, ledger: &Ledger) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "hash:              {}", ledger.hash());
        append!(buf, "sequence:          {}", ledger.sequence());
        append!(buf, "transaction count: {}", ledger.transaction_count());
        append!(buf, "operation count:   {}", ledger.operation_count());
        append!(buf, "total coins:       {}", ledger.total_coins());
        append!(buf, "fee pool:          {}", ledger.fee_pool());
        append!(buf, "base fee:          {}", ledger.base_fee_as_amount());
        append!(
            buf,
            "base reserve:      {}",
            ledger.base_reserve_as_amount()
        );
        append!(buf, "set size:          {}", ledger.max_tx_set_size());
        append!(buf, "version:           {}", ledger.protocol_version());
        append!(buf, "closed at:         {}", ledger.closed_at());
        Some(buf)
    }
}
