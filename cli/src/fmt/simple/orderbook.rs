use super::Simple;
use fmt::Render;
use stellar_client::resources::Orderbook;

impl Render<Orderbook> for Simple {
    fn render(&self, orderbook: &Orderbook) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "bids:");
        for bid in orderbook.bids() {
            append!(buf, "  {{");
            append!(buf, "    Amount:      {}", bid.amount());
            append!(buf, "    Price Ratio: {}", bid.price_ratio());
            append!(buf, "    Price:       {}", bid.price());
            append!(buf, "  }},");
        }
        append!(buf, "asks:");
        for ask in orderbook.asks() {
            append!(buf, "  {{");
            append!(buf, "    Amount:      {}", ask.amount());
            append!(buf, "    Price Ratio: {}", ask.price_ratio());
            append!(buf, "    Price:       {}", ask.price());
            append!(buf, "  }},");
        }
        append!(
            buf,
            "Base Asset:    {}",
            self.render(orderbook.base()).unwrap()
        );
        append!(
            buf,
            "Counter Asset: {}",
            self.render(orderbook.counter()).unwrap()
        );

        Some(buf)
    }
}
