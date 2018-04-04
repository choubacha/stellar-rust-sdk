use stellar_client::{endpoint::{transaction, Order}, error::Result, sync::{self, Client}};
use clap::ArgMatches;
use super::pager::Pager;

pub fn all<'a>(client: Client, matches: &'a ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);
    let endpoint = transaction::All::default()
        .order(Order::Desc)
        .limit(pager.horizon_page_limit() as u32);
    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    pager.paginate(iter, |result| match result {
        Ok(txn) => {
            println!("ID:             {}", txn.id());
            println!("source account: {}", txn.source_account());
            println!("created at:     {}", txn.created_at());
            println!("");
        }
        Err(err) => res = Err(err),
    });
    res
}
