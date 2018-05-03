use fmt::Render;
use super::Simple;
use stellar_client::resources::Orderbook;

impl Render<Orderbook> for Simple {
    fn render(&self, orderbook: &Orderbook) -> Option<String> {
        let mut buf = String::new();
        append_to_buffer!(buf, "bids:");
        for bid in orderbook.bids() {
            append_to_buffer!(buf, "  {{");
            append_to_buffer!(buf, "    Amount:      {}", bid.amount());
            append_to_buffer!(buf, "    Price Ratio: {}", bid.price_ratio());
            append_to_buffer!(buf, "    Price:       {}", bid.price());
            append_to_buffer!(buf, "  }},");
        }
        append_to_buffer!(buf, "asks:");
        for ask in orderbook.asks() {
            append_to_buffer!(buf, "  {{");
            append_to_buffer!(buf, "    Amount:      {}", ask.amount());
            append_to_buffer!(buf, "    Price Ratio: {}", ask.price_ratio());
            append_to_buffer!(buf, "    Price:       {}", ask.price());
            append_to_buffer!(buf, "  }},");
        }
        append_to_buffer!(
            buf,
            "Base Asset Type:      {}",
            orderbook.base().asset_type()
        );
        append_to_buffer!(buf, "Base Asset Code:      {}", orderbook.base().code());
        append_to_buffer!(buf, "Base Asset Issuer:    {}", orderbook.base().issuer());
        append_to_buffer!(
            buf,
            "Counter Asset Type:   {}",
            orderbook.counter().asset_type()
        );
        append_to_buffer!(buf, "Counter Asset Code:   {}", orderbook.counter().code());
        append_to_buffer!(
            buf,
            "Counter Asset Issuer: {}",
            orderbook.counter().issuer()
        );

        Some(buf)
    }
}
