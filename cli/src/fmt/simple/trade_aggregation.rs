use fmt::Render;
use super::Simple;
use stellar_client::resources::TradeAggregation;

impl Render<TradeAggregation> for Simple {
    fn render(&self, aggregation: &TradeAggregation) -> Option<String> {
        let mut buf = String::new();

        macro_rules! append {
            ($($args:tt)*) => {
                buf.push_str(&format!($($args)*));
                buf.push_str("\n");
            }
        }
        append!("timestamp:      {}", aggregation.started_at());
        append!("trade_count:    {}", aggregation.count());
        append!("base_volume:    {}", aggregation.base_volume());
        append!("counter_volume: {}", aggregation.counter_volume());
        append!("average:        {}", aggregation.average());
        append!("high:           {}", aggregation.high());
        append!("low:            {}", aggregation.low());
        append!("open:           {}", aggregation.open());
        append!("close:          {}", aggregation.close());
        Some(buf)
    }
}
