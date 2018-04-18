use stellar_client::{endpoint::ledger, error::Result, sync::{self, Client}};
use clap::ArgMatches;
use super::{cursor, ordering, pager::Pager};

pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = ledger::All::default();
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    pager.paginate(iter, |result| match result {
        Ok(ledger) => {
            println!("hash:              {}", ledger.hash());
            println!("sequence:          {}", ledger.sequence());
            println!("transaction count: {}", ledger.transaction_count());
            println!("operation count:   {}", ledger.operation_count());
            println!("closed at:         {}", ledger.closed_at());
            println!();
        }
        Err(err) => res = Err(err),
    });
    res
}
