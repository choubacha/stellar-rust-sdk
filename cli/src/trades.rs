use chrono::{DateTime, Utc};
use clap::ArgMatches;
use error::Result;
use fmt::{Formatter, Simple};
use resolution::Resolution;
use stellar_client::{endpoint::trade, resources::AssetIdentifier, sync::{self, Client}};
use super::{cursor, ordering, pager::Pager};

pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = {
        let mut endpoint = trade::All::default();

        if let Some(offer_id) = matches.value_of("offer_id") {
            let offer_id = offer_id
                .parse::<u32>()
                .map_err(|_| String::from("Offer Id should be a valid u32 integer"))?;
            endpoint = endpoint.with_offer_id(offer_id);
        };
        if let (Some(base_asset_type), Some(counter_asset_type)) = (
            matches.value_of("base_asset_type"),
            matches.value_of("counter_asset_type"),
        ) {
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
            let base_asset =
                AssetIdentifier::new(base_asset_type, base_asset_code, base_asset_issuer)?;
            let counter_asset =
                AssetIdentifier::new(counter_asset_type, counter_asset_code, counter_asset_issuer)?;
            endpoint = endpoint.with_asset_pair(base_asset, counter_asset);
        }
        endpoint = pager.assign(endpoint);
        endpoint = cursor::assign_from_arg(matches, endpoint);
        ordering::assign_from_arg(matches, endpoint)
    };

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    pager.paginate(iter, |result| match result {
        Ok(trade) => {
            println!("id:                   {}", trade.id());
            println!("offer_id:             {}", trade.offer_id());
            println!("paging_token:         {}", trade.paging_token());
            println!("ledger_close_time:    {}", trade.closed_at());
            println!("base_account:         {}", trade.base_account());
            println!("base_amount:          {}", trade.base_amount());
            println!("base_asset_code:      {}", trade.base_asset().code());
            println!("base_asset_issuer:    {}", trade.base_asset().issuer());
            println!("counter_amount:       {}", trade.counter_amount());
            println!("counter_account:      {}", trade.counter_account());
            println!("counter_asset_code:   {}", trade.counter_asset().code());
            println!("counter_asset_issuer: {}", trade.counter_asset().issuer());
            println!("price:                {}", trade.price());
            println!("seller:               {}", trade.selling_account());
            println!();
        }
        Err(err) => res = Err(err.into()),
    });
    res
}

pub fn aggregations(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

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
        let mut endpoint = trade::Aggregations::new(&base_asset, &counter_asset);
        let resolution = matches
            .value_of("resolution")
            .expect("Resolution is a required field");
        let resolution: Resolution = resolution.parse()?;
        endpoint = endpoint.with_resolution(resolution.to_ms());
        let start_time = matches
            .value_of("start_time")
            .expect("Start time is a required field")
            .parse::<DateTime<Utc>>()
            .map_err(|_| {
                String::from("Start time should be in ISO 8601 format, ex: x2017-11-28T12:00:09Z")
            })?
            .timestamp() as u64 * 1_000;
        endpoint = endpoint.with_start_time(start_time);

        let end_time = matches
            .value_of("end_time")
            .expect("End time is a required field")
            .parse::<DateTime<Utc>>()
            .map_err(|_| {
                String::from("Start time should be in ISO 8601 format, ex: x2017-11-28T12:00:09Z")
            })?
            .timestamp() as u64 * 1_000;
        endpoint = endpoint.with_end_time(end_time);
        endpoint = pager.assign(endpoint);
        endpoint
    };

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple);
    pager.paginate(iter, |result| match result {
        Ok(aggregation) => fmt.render(&aggregation),
        Err(err) => res = Err(err.into()),
    });
    let _ = fmt.stop();
    res
}
