use asset_identifier;
use clap::ArgMatches;
use error::Result;
use fmt::{Formatter, Simple};
use stellar_client::{endpoint::{orderbook, Limit},
                     sync::Client};

pub fn details(client: &Client, matches: &ArgMatches) -> Result<()> {
    let endpoint = {
        let base_str = matches
            .value_of("base")
            .expect("Base asset is a required field");
        let counter_str = matches
            .value_of("counter")
            .expect("Counter asset is a required field");
        let base = asset_identifier::from_str(base_str)?;
        let counter = asset_identifier::from_str(counter_str)?;
        let mut endpoint = orderbook::Details::for_asset_pair(base, counter);
        if let Some(limit) = matches.value_of("limit") {
            let limit = limit.parse::<u32>()?;
            endpoint = endpoint.with_limit(limit);
        }
        endpoint
    };

    let orderbook = client.request(endpoint)?;
    Formatter::start_stdout(Simple::new()).render(&orderbook);
    Ok(())
}
