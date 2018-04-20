use stellar_client::{endpoint::trades, sync::{self, Client}};
use stellar_resources::AssetIdentifier;
use clap::ArgMatches;
use super::{cursor, ordering, pager::Pager};
use error::Result;

pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = {
        let mut endpoint = trades::All::default();

        if let Some(offer_id) = matches.value_of("offer_id") {
            let offer_id = offer_id.parse::<u32>().unwrap();
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
