use fmt::Render;
use super::Simple;
use stellar_client::resources::Ledger;

impl Render<Ledger> for Simple {
    fn render(&self, ledger: &Ledger) -> Option<String> {
        let mut buf = String::new();
        append_to_buffer!(buf, "hash:              {}", ledger.hash());
        append_to_buffer!(buf, "sequence:          {}", ledger.sequence());
        append_to_buffer!(buf, "transaction count: {}", ledger.transaction_count());
        append_to_buffer!(buf, "operation count:   {}", ledger.operation_count());
        append_to_buffer!(buf, "total coins:       {}", ledger.total_coins());
        append_to_buffer!(buf, "fee pool:          {}", ledger.fee_pool());
        append_to_buffer!(buf, "base fee:          {}", ledger.base_fee_as_amount());
        append_to_buffer!(
            buf,
            "base reserve:      {}",
            ledger.base_reserve_as_amount()
        );
        append_to_buffer!(buf, "set size:          {}", ledger.max_tx_set_size());
        append_to_buffer!(buf, "version:           {}", ledger.protocol_version());
        append_to_buffer!(buf, "closed at:         {}", ledger.closed_at());
        Some(buf)
    }
}
