use clap::ArgMatches;
use error::Result;
use fmt::{Formatter, Simple};
use stellar_client::{endpoint::{orderbook, Limit}, resources::AssetIdentifier, sync::Client};

pub fn details(client: &Client, matches: &ArgMatches) -> Result<()> {
    let endpoint = {
        let base_asset_type = matches
            .value_of("base_asset_type")
            .expect("Base asset type is a required field");
        let counter_asset_type = matches
            .value_of("counter_asset_type")
            .expect("Counter asset type is a required field");
        let base_asset_code = matches
            .value_of("base_asset_code")
            .map(|code| code.to_string());
        let base_asset_issuer = matches
            .value_of("base_asset_issuer")
            .map(|issuer| issuer.to_string());
        let counter_asset_code = matches
            .value_of("counter_asset_code")
            .map(|code| code.to_string());
        let counter_asset_issuer = matches
            .value_of("counter_asset_issuer")
            .map(|issuer| issuer.to_string());
        let base_asset = AssetIdentifier::new(base_asset_type, base_asset_code, base_asset_issuer)?;
        let counter_asset =
            AssetIdentifier::new(counter_asset_type, counter_asset_code, counter_asset_issuer)?;
        let mut endpoint = orderbook::Details::for_asset_pair(base_asset, counter_asset);
        if let Some(limit) = matches.value_of("limit") {
            let limit = limit
                .parse::<u32>()
                .map_err(|_| String::from("Limitshould be a valid u32 integer"))?;
            endpoint = endpoint.with_limit(limit);
        }
        endpoint
    };

    let orderbook = client.request(endpoint)?;
    Formatter::start_stdout(Simple).render(&orderbook);
    Ok(())
}
