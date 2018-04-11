use stellar_client::{endpoint::{transaction, Limit, Order}, error::Result, sync::{self, Client}};
use clap::ArgMatches;
use super::{cursor, ordering, pager::Pager};

pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);
    let endpoint = {
        let endpoint = transaction::All::default()
            .with_order(ordering::from_arg(matches))
            .with_limit(pager.horizon_page_limit() as u32);
        cursor::assign_from_arg(matches, endpoint)
    };
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
