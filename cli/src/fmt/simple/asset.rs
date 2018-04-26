use fmt::Render;
use super::Simple;
use stellar_client::resources::Asset;

impl Render<Asset> for Simple {
    fn render(&self, asset: &Asset) -> Option<String> {
        let mut buf = String::new();

        macro_rules! append {
            ($($args:tt)*) => {
                buf.push_str(&format!($($args)*));
                buf.push_str("\n");
            }
        }

        append!("Code:         {}", asset.code());
        append!("Type:         {}", asset.asset_type());
        append!("Issuer:       {}", asset.issuer());
        append!("Amount:       {}", asset.amount());
        append!("Num accounts: {}", asset.num_accounts());
        append!("Flags:");
        if asset.is_auth_required() {
            append!("  auth is required");
        }
        if asset.is_auth_revocable() {
            append!("  auth is revocable");
        }
        Some(buf)
    }
}
