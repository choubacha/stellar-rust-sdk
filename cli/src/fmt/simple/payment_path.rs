use super::Simple;
use fmt::Render;
use stellar_client::resources::PaymentPath;

impl Render<PaymentPath> for Simple {
    fn render(&self, payment_path: &PaymentPath) -> Option<String> {
        let mut buf = String::new();
        append!(
            buf,
            "Destination amount: {}",
            payment_path.destination_amount()
        );
        append!(
            buf,
            "Destination asset:  {}",
            self.render(payment_path.destination_asset()).unwrap()
        );
        append!(buf, "Source amount:      {}", payment_path.source_amount());
        append!(
            buf,
            "Source asset:       {}",
            self.render(payment_path.source_asset()).unwrap()
        );
        if payment_path.path().len() > 0 {
            append!(buf, "PATH:");
            for asset in payment_path.path().iter() {
                indent!(buf, self, "{}\n", self.render(asset).unwrap());
            }
        } else {
            append!(buf, "PATH: []");
        }
        Some(buf)
    }
}
