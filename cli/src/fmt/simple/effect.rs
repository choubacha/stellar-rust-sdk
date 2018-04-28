use fmt::Render;
use super::Simple;
use stellar_client::resources::{AssetIdentifier, effect::{Effect, EffectKind as Kind}};

impl Render<Effect> for Simple {
    fn render(&self, effect: &Effect) -> Option<String> {
        let mut buf = String::new();

        append_to_buffer!(buf, "id:   {}", effect.id());
        append_to_buffer!(buf, "kind: {}", effect.kind_name());
        append_to_buffer!(buf, "details:");

        Some(match *effect.kind() {
            Kind::Account(ref kind) => account::render(buf, kind),
            Kind::Signer(ref kind) => signer::render(buf, kind),
            Kind::Trustline(ref kind) => trustline::render(buf, kind),
            Kind::Trade(ref kind) => trade::render(buf, kind),
            Kind::Data(ref kind) => data::render(buf, kind),
        })
    }
}

fn render_asset(id: &AssetIdentifier) -> String {
    if id.is_native() {
        id.code().to_string()
    } else {
        format!("{}-{}", id.code(), id.issuer())
    }
}

mod account {
    use super::*;
    use stellar_client::resources::effect::account::Kind;

    pub fn render(mut buf: String, kind: &Kind) -> String {
        match *kind {
            Kind::Created(ref effect) => {
                append_to_buffer!(buf, "  account:          {}", effect.account());
                append_to_buffer!(buf, "  starting_balance: {}", effect.starting_balance());
            }
            Kind::Credited(ref effect) => {
                append_to_buffer!(buf, "  account: {}", effect.account());
                append_to_buffer!(buf, "  asset:   {}", render_asset(effect.asset()));
                append_to_buffer!(buf, "  amount:  {}", effect.amount());
            }
            Kind::Removed(ref effect) => {
                append_to_buffer!(buf, "  account: {}", effect.account());
            }
            Kind::Debited(ref effect) => {
                append_to_buffer!(buf, "  account: {}", effect.account());
                append_to_buffer!(buf, "  asset:   {}", render_asset(effect.asset()));
                append_to_buffer!(buf, "  amount:  {}", effect.amount());
            }
            Kind::ThresholdsUpdated(ref effect) => {
                append_to_buffer!(buf, "  account: {}", effect.account());
                append_to_buffer!(buf, "  low:     {}", effect.low());
                append_to_buffer!(buf, "  med:     {}", effect.med());
                append_to_buffer!(buf, "  high:    {}", effect.high());
            }
            Kind::HomeDomainUpdated(ref effect) => {
                append_to_buffer!(buf, "  account:     {}", effect.account());
                append_to_buffer!(buf, "  home domain: {}", effect.home_domain());
            }
            Kind::FlagsUpdated(ref effect) => {
                append_to_buffer!(buf, "  account:     {}", effect.account());
                append_to_buffer!(buf, "  flags:");
                if effect.flags().is_auth_required() {
                    append_to_buffer!(buf, "    auth is required");
                }
                if effect.flags().is_auth_revocable() {
                    append_to_buffer!(buf, "    auth is revocable");
                }
            }
        }
        buf
    }
}

mod signer {
    use stellar_client::resources::effect::signer::Kind;

    pub fn render(mut buf: String, kind: &Kind) -> String {
        match *kind {
            Kind::Created(ref effect) => {
                append_to_buffer!(buf, "  account:    {}", effect.account());
                append_to_buffer!(buf, "  public key: {}", effect.public_key());
                append_to_buffer!(buf, "  weight:     {}", effect.weight());
            }
            Kind::Removed(ref effect) => {
                append_to_buffer!(buf, "  account:    {}", effect.account());
                append_to_buffer!(buf, "  public key: {}", effect.public_key());
                append_to_buffer!(buf, "  weight:     {}", effect.weight());
            }
            Kind::Updated(ref effect) => {
                append_to_buffer!(buf, "  account:    {}", effect.account());
                append_to_buffer!(buf, "  public key: {}", effect.public_key());
                append_to_buffer!(buf, "  weight:     {}", effect.weight());
            }
        }
        buf
    }
}

mod trustline {
    use super::*;
    use stellar_client::resources::effect::trustline::Kind;

    pub fn render(mut buf: String, kind: &Kind) -> String {
        match *kind {
            Kind::Authorized(ref effect) => {
                append_to_buffer!(buf, "  account: {}", effect.account());
                append_to_buffer!(buf, "  asset:   {}", render_asset(effect.asset()));
            }
            Kind::Deauthorized(ref effect) => {
                append_to_buffer!(buf, "  account: {}", effect.account());
                append_to_buffer!(buf, "  asset:   {}", render_asset(effect.asset()));
            }
            Kind::Removed(ref effect) => {
                append_to_buffer!(buf, "  account: {}", effect.account());
                append_to_buffer!(buf, "  limit:   {}", effect.limit());
                append_to_buffer!(buf, "  asset:   {}", render_asset(effect.asset()));
            }
            Kind::Updated(ref effect) => {
                append_to_buffer!(buf, "  account: {}", effect.account());
                append_to_buffer!(buf, "  limit:   {}", effect.limit());
                append_to_buffer!(buf, "  asset:   {}", render_asset(effect.asset()));
            }
            Kind::Created(ref effect) => {
                append_to_buffer!(buf, "  account: {}", effect.account());
                append_to_buffer!(buf, "  limit:   {}", effect.limit());
                append_to_buffer!(buf, "  asset:   {}", render_asset(effect.asset()));
            }
        }
        buf
    }
}

mod trade {
    use super::*;
    use stellar_client::resources::effect::trade::Kind;

    pub fn render(mut buf: String, kind: &Kind) -> String {
        match *kind {
            Kind::Trade(ref effect) => {
                append_to_buffer!(buf, "  account:      {}", effect.account());
                append_to_buffer!(buf, "  seller:       {}", effect.seller());
                append_to_buffer!(buf, "  offer id:     {}", effect.offer_id());
                append_to_buffer!(buf, "  sold amount:  {}", effect.sold_amount());
                append_to_buffer!(buf, "  sold asset:   {}", render_asset(effect.sold_asset()));
                append_to_buffer!(buf, "  bough amount: {}", effect.bought_amount());
                append_to_buffer!(
                    buf,
                    "  bought asset: {}",
                    render_asset(effect.bought_asset())
                );
            }
        }
        buf
    }
}

mod data {
    use stellar_client::resources::effect::data::Kind;

    pub fn render(mut buf: String, kind: &Kind) -> String {
        match *kind {
            Kind::Created(ref effect) => {
                append_to_buffer!(buf, "  account:    {}", effect.account());
            }
            Kind::Removed(ref effect) => {
                append_to_buffer!(buf, "  account:    {}", effect.account());
            }
            Kind::Updated(ref effect) => {
                append_to_buffer!(buf, "  account:    {}", effect.account());
            }
        }
        buf
    }
}
