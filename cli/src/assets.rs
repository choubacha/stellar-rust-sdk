use stellar_client::{endpoint::asset, error::Result, sync::{self, Client}};
use clap::ArgMatches;
use super::{cursor, ordering, pager::Pager};

/// Using a client and the arguments from the command line, iterates over the results
/// and displays them to the end user.
pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = {
        let mut endpoint = asset::All::default();

        if let Some(code) = matches.value_of("code") {
            endpoint = endpoint.with_asset_code(code);
        }
        if let Some(issuer) = matches.value_of("issuer") {
            endpoint = endpoint.with_asset_issuer(issuer);
        }

        endpoint = pager.assign(endpoint);
        endpoint = ordering::assign_from_arg(matches, endpoint);
        cursor::assign_from_arg(matches, endpoint)
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
            println!();
        }
        Err(err) => res = Err(err),
    });
    res
}
