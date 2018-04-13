use clap::ArgMatches;
use stellar_client::{sync, endpoint::account, error::Result, sync::Client};
use super::{cursor, ordering, pager::Pager};

pub fn details(client: &Client, matches: &ArgMatches) -> Result<()> {
    let id = matches.value_of("ID").expect("ID is required");
    let endpoint = account::Details::new(id);
    let account = client.request(endpoint)?;

    println!("ID:       {}", account.id());
    println!("Sequence: {}", account.sequence());

    Ok(())
}

pub fn transactions(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let id = matches.value_of("ID").expect("ID is required");
    let endpoint = account::Transactions::new(id);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    pager.paginate(iter, |result| match result {
        Ok(txn) => {
            println!("ID:             {}", txn.id());
            println!("source account: {}", txn.source_account());
            println!("created at:     {}", txn.created_at());
            println!();
        }
        Err(err) => res = Err(err),
    });
    res
}
