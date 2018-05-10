use super::Simple;
use fmt::Render;
use stellar_client::resources::TradeAggregation;

impl Render<TradeAggregation> for Simple {
    fn render(&self, aggregation: &TradeAggregation) -> Option<String> {
        let mut buf = String::new();
        append_to_buffer!(buf, "timestamp:      {}", aggregation.started_at());
        append_to_buffer!(buf, "trade_count:    {}", aggregation.count());
        append_to_buffer!(buf, "base_volume:    {}", aggregation.base_volume());
        append_to_buffer!(buf, "counter_volume: {}", aggregation.counter_volume());
        append_to_buffer!(buf, "average:        {}", aggregation.average());
        append_to_buffer!(buf, "high:           {}", aggregation.high());
        append_to_buffer!(buf, "low:            {}", aggregation.low());
        append_to_buffer!(buf, "open:           {}", aggregation.open());
        append_to_buffer!(buf, "close:          {}", aggregation.close());
        Some(buf)
    }
}
