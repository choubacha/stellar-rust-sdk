use super::Simple;
use fmt::Render;
use stellar_client::resources::Datum;

impl Render<Datum> for Simple {
    fn render(&self, datum: &Datum) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "value:       {}", datum.value());
        Some(buf)
    }
}
