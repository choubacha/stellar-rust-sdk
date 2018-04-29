use super::Simple;
use fmt::Render;
use stellar_client::resources::{Asset, AssetIdentifier, Flags};

impl Render<Asset> for Simple {
    fn render(&self, asset: &Asset) -> Option<String> {
        let mut buf = String::new();
        append!(
            buf,
            "Asset:        {}",
            self.render(asset.identifier()).unwrap()
        );
        append!(buf, "Amount:       {}", asset.amount());
        append!(buf, "Num Accounts: {}", asset.num_accounts());
        append!(buf, "Flags:");
        nest!(buf, self, &asset.flags());
        Some(buf)
    }
}

impl Render<AssetIdentifier> for Simple {
    fn render(&self, asset_id: &AssetIdentifier) -> Option<String> {
        let asset_str = if asset_id.is_native() {
            asset_id.code().to_string()
        } else {
            format!("{}-{}", asset_id.code(), asset_id.issuer())
        };
        Some(asset_str)
    }
}

impl Render<Flags> for Simple {
    fn render(&self, flags: &Flags) -> Option<String> {
        let mut buf = String::new();
        if flags.is_auth_required() {
            append!(buf, "auth is required");
        }
        if flags.is_auth_revocable() {
            append!(buf, "auth is revocable");
        }
        Some(buf)
    }
}
