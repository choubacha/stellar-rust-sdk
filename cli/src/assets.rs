use stellar_client::{endpoint::{asset, Order}, error::Result, sync::{self, Client}};
use clap::ArgMatches;
use super::pager::Pager;

pub fn all<'a>(client: Client, matches: &'a ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = {
        let mut endpoint = asset::All::default()
            .order(Order::Asc)
            .limit(pager.horizon_page_limit() as u32);

        if let Some(code) = matches.value_of("code") {
            endpoint = endpoint.asset_code(code)
        }
        if let Some(issuer) = matches.value_of("issuer") {
            endpoint = endpoint.asset_issuer(issuer)
        }
        endpoint
    };
    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    pager.paginate(iter, |result| match result {
        Ok(asset) => {
            println!("Code:           {}", asset.code());
            println!("Type:           {}", asset.asset_type());
            println!("Issuer:         {}", asset.issuer());
            println!("Amount:         {}", asset.amount());
            println!("Num accounts:   {}", asset.num_accounts());
            println!("Flags:");
            if asset.is_auth_required() {
                println!("  auth is required");
            }
            if asset.is_auth_revocable() {
                println!("  auth is revocable");
            }
            println!("");
        }
        Err(err) => res = Err(err),
    });
    res
}
