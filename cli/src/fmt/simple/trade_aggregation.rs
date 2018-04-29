use super::Simple;
use fmt::Render;
use stellar_client::resources::TradeAggregation;

impl Render<TradeAggregation> for Simple {
    fn render(&self, aggregation: &TradeAggregation) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "timestamp:      {}", aggregation.started_at());
        append!(buf, "trade_count:    {}", aggregation.count());
        append!(buf, "base_volume:    {}", aggregation.base_volume());
        append!(buf, "counter_volume: {}", aggregation.counter_volume());
        append!(buf, "average:        {}", aggregation.average());
        append!(buf, "high:           {}", aggregation.high());
        append!(buf, "low:            {}", aggregation.low());
        append!(buf, "open:           {}", aggregation.open());
        append!(buf, "close:          {}", aggregation.close());
        Some(buf)
    }
}
