use fmt::Render;
use super::Simple;
use stellar_client::resources::{AssetIdentifier, Offer};

fn render_asset(id: &AssetIdentifier) -> String {
    if id.is_native() {
        id.code().to_string()
    } else {
        format!("{}-{}", id.code(), id.issuer())
    }
}

impl Render<Offer> for Simple {
    fn render(&self, offer: &Offer) -> Option<String> {
        let mut buf = String::new();
        append_to_buffer!(buf, "ID:           {}", offer.id());
        append_to_buffer!(buf, "Seller:       {}", offer.seller());
        append_to_buffer!(buf, "Selling:      {}", render_asset(offer.selling()));
        append_to_buffer!(buf, "Buying:       {}", render_asset(offer.buying()));
        append_to_buffer!(buf, "Amount:       {}", offer.amount());
        append_to_buffer!(buf, "Price:        {}", offer.price());
        Some(buf)
    }
}
