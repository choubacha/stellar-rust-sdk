use super::Simple;
use fmt::Render;
use stellar_client::resources::Trade;

impl Render<Trade> for Simple {
    fn render(&self, trade: &Trade) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "id:                {}", trade.id());
        append!(buf, "offer_id:          {}", trade.offer_id());
        append!(buf, "paging_token:      {}", trade.paging_token());
        append!(buf, "ledger_close_time: {}", trade.closed_at());
        append!(buf, "base_account:      {}", trade.base_account());
        append!(buf, "base_amount:       {}", trade.base_amount());
        append!(
            buf,
            "base_asset:        {}",
            self.render(trade.base_asset()).unwrap()
        );
        append!(buf, "counter_amount:    {}", trade.counter_amount());
        append!(buf, "counter_account:   {}", trade.counter_account());
        append!(
            buf,
            "counter_asset:     {}",
            self.render(trade.counter_asset()).unwrap()
        );
        append!(buf, "price:             {}", trade.price());
        append!(buf, "seller:            {}", trade.selling_account());

        Some(buf)
    }
}
