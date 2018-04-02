use clap::ArgMatches;
use stellar_client::{sync, endpoint::{account, Order}, error::Result, sync::Client};
use super::utils;

pub fn details<'a>(client: Client, matches: &'a ArgMatches) -> Result<()> {
    let id = matches.value_of("ID").expect("ID is required");
    let endpoint = account::Details::new(id);
    let account = client.request(endpoint)?;

    println!("ID:       {}", account.id());
    println!("Sequence: {}", account.sequence());

    Ok(())
}

pub fn transactions<'a>(client: Client, matches: &'a ArgMatches) -> Result<()> {
    let id = matches.value_of("ID").expect("ID is required");
    let endpoint = account::Transactions::new(id).order(Order::Desc);
    let iter = sync::Iter::new(&client, endpoint);

    for (i, result) in iter.enumerate() {
        let txn = result?;

        println!("ID:             {}", txn.id());
        println!("source account: {}", txn.source_account());
        println!("created at:     {}", txn.created_at());
        println!("");

        if (i + 1) % 10 == 0 && !utils::next_page() {
            break;
        }
    }
    Ok(())
}
