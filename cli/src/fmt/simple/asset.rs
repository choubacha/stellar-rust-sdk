use super::Simple;
use fmt::Render;
use stellar_client::resources::Asset;

impl Render<Asset> for Simple {
    fn render(&self, asset: &Asset) -> Option<String> {
        let mut buf = String::new();
        append_to_buffer!(buf, "Code:         {}", asset.code());
        append_to_buffer!(buf, "Type:         {}", asset.asset_type());
        append_to_buffer!(buf, "Issuer:       {}", asset.issuer());
        append_to_buffer!(buf, "Amount:       {}", asset.amount());
        append_to_buffer!(buf, "Num accounts: {}", asset.num_accounts());
        append_to_buffer!(buf, "Flags:");
        if asset.is_auth_required() {
            append_to_buffer!(buf, "  auth is required");
        }
        if asset.is_auth_revocable() {
            append_to_buffer!(buf, "  auth is revocable");
        }
        Some(buf)
    }
}
