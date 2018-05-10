use super::{cursor, ordering, pager::Pager};
use clap::ArgMatches;
use error::Result;
use fmt::{Formatter, Simple};
use stellar_client::{endpoint::transaction, sync::{self, Client}};

pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = transaction::All::default();
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple);
    pager.paginate(iter, |result| match result {
        Ok(txn) => fmt.render(&txn),
        Err(err) => res = Err(err.into()),
    });
    let _ = fmt.stop();
    res
}

pub fn payments(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);
    let hash = matches
        .value_of("Hash")
        .expect("Transaction identifier hash is required");

    let endpoint = transaction::Payments::new(hash);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple);
    pager.paginate(iter, |result| match result {
        Ok(operation) => fmt.render(&operation),
        Err(err) => res = Err(err.into()),
    });
    res
}

pub fn details(client: &Client, matches: &ArgMatches) -> Result<()> {
    let hash = matches
        .value_of("HASH")
        .expect("Transaction identifier hash is required");
    let endpoint = transaction::Details::new(&hash);
    let transaction = client.request(endpoint)?;
    let mut fmt = Formatter::start_stdout(Simple);
    fmt.render(&transaction);
    Ok(())
}

pub fn operations(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);
    let hash = matches
        .value_of("Hash")
        .expect("Transaction identifier hash is required");

    let endpoint = transaction::Operations::new(hash);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    pager.paginate(iter, |result| match result {
        Ok(op) => {
            println!("ID:   {}", op.id());
            println!("Type: {}", op.kind_name());
            println!();
        }
        Err(err) => res = Err(err.into()),
    });
    res
}

pub fn effects(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);
    let hash = matches
        .value_of("Hash")
        .expect("Transaction identifier hash is required");

    let endpoint = transaction::Effects::new(hash);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple);
    pager.paginate(iter, |result| match result {
        Ok(txn) => fmt.render(&txn),
        Err(err) => res = Err(err.into()),
    });
    let _ = fmt.stop();
    res
}
