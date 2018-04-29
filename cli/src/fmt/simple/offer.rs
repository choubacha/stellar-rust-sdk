use super::Simple;
use fmt::Render;
use stellar_client::resources::Offer;

impl Render<Offer> for Simple {
    fn render(&self, offer: &Offer) -> Option<String> {
        let mut buf = String::new();
        append!(buf, "ID:           {}", offer.id());
        append!(buf, "Seller:       {}", offer.seller());
        append!(
            buf,
            "Selling:      {}",
            self.render(offer.selling()).unwrap()
        );
        append!(
            buf,
            "Buying:       {}",
            self.render(offer.buying()).unwrap()
        );
        append!(buf, "Amount:       {}", offer.amount());
        append!(buf, "Price:        {}", offer.price());
        Some(buf)
    }
}
