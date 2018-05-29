use super::{cursor, ordering, pager::Pager};
use clap::ArgMatches;
use error::Result;
use fmt::{Formatter, Simple};
use stellar_client::{endpoint::ledger,
                     sync::{self, Client}};

pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = ledger::All::default();
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple::new());
    pager.paginate(iter, |result| match result {
        Ok(ledger) => fmt.render(&ledger),
        Err(err) => res = Err(err.into()),
    });
    fmt.stop();
    res
}

pub fn details(client: &Client, matches: &ArgMatches) -> Result<()> {
    let sequence = matches
        .value_of("sequence")
        .expect("Ledger sequence is required")
        .parse::<u32>()
        .map_err(|_| String::from("Payment sequence should be a valid u32 integer"))?;

    let endpoint = ledger::Details::new(sequence);
    let ledger = client.request(endpoint)?;

    Formatter::start_stdout(Simple::new()).render(&ledger);

    Ok(())
}

pub fn effects(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let sequence = matches
        .value_of("sequence")
        .expect("Ledger sequence is required");

    let sequence = sequence
        .parse::<u32>()
        .map_err(|_| String::from("Payment sequence should be a valid u32 integer"))?;
    let endpoint = ledger::Effects::new(sequence);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple::new());
    pager.paginate(iter, |result| match result {
        Ok(effect) => fmt.render(&effect),
        Err(err) => res = Err(err.into()),
    });
    let _ = fmt.stop();
    res
}

pub fn payments(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);
    let sequence = matches
        .value_of("sequence")
        .expect("Ledger sequence is required");

    let sequence = sequence
        .parse::<u32>()
        .map_err(|_| String::from("Ledger sequence should be a valid u32 integer"))?;
    let endpoint = ledger::Payments::new(sequence);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple::new());
    pager.paginate(iter, |result| match result {
        Ok(operation) => fmt.render(&operation),
        Err(err) => res = Err(err.into()),
    });
    res
}

pub fn operations(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);
    let sequence = matches
        .value_of("sequence")
        .expect("Ledger sequence is required");

    let sequence = sequence
        .parse::<u32>()
        .map_err(|_| String::from("Ledger sequence should be a valid u32 integer"))?;
    let endpoint = ledger::Operations::new(sequence);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple::new());
    pager.paginate(iter, |result| match result {
        Ok(operation) => fmt.render(&operation),
        Err(err) => res = Err(err.into()),
    });
    res
}

pub fn transactions(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);
    let sequence = matches
        .value_of("sequence")
        .expect("Ledger sequence is required");

    let sequence = sequence
        .parse::<u32>()
        .map_err(|_| String::from("Payment sequence should be a valid u32 integer"))?;
    let endpoint = ledger::Transactions::new(sequence);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple::new());
    pager.paginate(iter, |result| match result {
        Ok(transaction) => fmt.render(&transaction),
        Err(err) => res = Err(err.into()),
    });
    res
}
