use clap::ArgMatches;
use stellar_client::{sync, endpoint::{account, Limit}, error::Result, sync::Client};
use super::{cursor, ordering, pager::Pager};

pub fn details<'a>(client: Client, matches: &'a ArgMatches) -> Result<()> {
    let id = matches.value_of("ID").expect("ID is required");
    let endpoint = account::Details::new(id);
    let account = client.request(endpoint)?;

    println!("ID:       {}", account.id());
    println!("Sequence: {}", account.sequence());

    Ok(())
}

pub fn transactions<'a>(client: Client, matches: &'a ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let id = matches.value_of("ID").expect("ID is required");
    let endpoint = {
        let endpoint = account::Transactions::new(id)
            .order(ordering::from_arg(matches))
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
            println!("");
        }
        Err(err) => res = Err(err),
    });
    res
}
