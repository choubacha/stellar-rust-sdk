use super::Simple;
use fmt::Render;
use stellar_client::resources::{effect::{Effect, EffectKind as Kind},
                                AssetIdentifier};

impl Render<Effect> for Simple {
    fn render(&self, effect: &Effect) -> Option<String> {
        let mut buf = String::new();

        append!(buf, "id:   {}", effect.id());
        append!(buf, "kind: {}", effect.kind_name());
        append!(buf, "details:");

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
                append!(buf, "  account:          {}", effect.account());
                append!(buf, "  starting_balance: {}", effect.starting_balance());
            }
            Kind::Credited(ref effect) => {
                append!(buf, "  account: {}", effect.account());
                append!(buf, "  asset:   {}", render_asset(effect.asset()));
                append!(buf, "  amount:  {}", effect.amount());
            }
            Kind::Removed(ref effect) => {
                append!(buf, "  account: {}", effect.account());
            }
            Kind::Debited(ref effect) => {
                append!(buf, "  account: {}", effect.account());
                append!(buf, "  asset:   {}", render_asset(effect.asset()));
                append!(buf, "  amount:  {}", effect.amount());
            }
            Kind::ThresholdsUpdated(ref effect) => {
                append!(buf, "  account: {}", effect.account());
                append!(buf, "  low:     {}", effect.low());
                append!(buf, "  med:     {}", effect.med());
                append!(buf, "  high:    {}", effect.high());
            }
            Kind::HomeDomainUpdated(ref effect) => {
                append!(buf, "  account:     {}", effect.account());
                append!(buf, "  home domain: {}", effect.home_domain());
            }
            Kind::FlagsUpdated(ref effect) => {
                append!(buf, "  account:     {}", effect.account());
                append!(buf, "  flags:");
                if effect.flags().is_auth_required() {
                    append!(buf, "    auth is required");
                }
                if effect.flags().is_auth_revocable() {
                    append!(buf, "    auth is revocable");
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
                append!(buf, "  account:    {}", effect.account());
                append!(buf, "  public key: {}", effect.public_key());
                append!(buf, "  weight:     {}", effect.weight());
            }
            Kind::Removed(ref effect) => {
                append!(buf, "  account:    {}", effect.account());
                append!(buf, "  public key: {}", effect.public_key());
                append!(buf, "  weight:     {}", effect.weight());
            }
            Kind::Updated(ref effect) => {
                append!(buf, "  account:    {}", effect.account());
                append!(buf, "  public key: {}", effect.public_key());
                append!(buf, "  weight:     {}", effect.weight());
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
                append!(buf, "  account: {}", effect.account());
                append!(buf, "  asset:   {}", render_asset(effect.asset()));
            }
            Kind::Deauthorized(ref effect) => {
                append!(buf, "  account: {}", effect.account());
                append!(buf, "  asset:   {}", render_asset(effect.asset()));
            }
            Kind::Removed(ref effect) => {
                append!(buf, "  account: {}", effect.account());
                append!(buf, "  limit:   {}", effect.limit());
                append!(buf, "  asset:   {}", render_asset(effect.asset()));
            }
            Kind::Updated(ref effect) => {
                append!(buf, "  account: {}", effect.account());
                append!(buf, "  limit:   {}", effect.limit());
                append!(buf, "  asset:   {}", render_asset(effect.asset()));
            }
            Kind::Created(ref effect) => {
                append!(buf, "  account: {}", effect.account());
                append!(buf, "  limit:   {}", effect.limit());
                append!(buf, "  asset:   {}", render_asset(effect.asset()));
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
                append!(buf, "  account:      {}", effect.account());
                append!(buf, "  seller:       {}", effect.seller());
                append!(buf, "  offer id:     {}", effect.offer_id());
                append!(buf, "  sold amount:  {}", effect.sold_amount());
                append!(buf, "  sold asset:   {}", render_asset(effect.sold_asset()));
                append!(buf, "  bough amount: {}", effect.bought_amount());
                append!(
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
                append!(buf, "  account:    {}", effect.account());
            }
            Kind::Removed(ref effect) => {
                append!(buf, "  account:    {}", effect.account());
            }
            Kind::Updated(ref effect) => {
                append!(buf, "  account:    {}", effect.account());
            }
        }
        buf
    }
}
