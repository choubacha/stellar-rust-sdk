use clap::ArgMatches;
use stellar_client::{sync, endpoint::account, sync::Client};
use super::{cursor, ordering, pager::Pager};
use fmt::{Formatter, Simple};
use error::Result;

pub fn details(client: &Client, matches: &ArgMatches) -> Result<()> {
    let id = matches.value_of("ID").expect("ID is required");
    let endpoint = account::Details::new(id);
    let account = client.request(endpoint)?;

    Formatter::start_stdout(Simple).render(&account);

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
    let mut fmt = Formatter::start_stdout(Simple);
    pager.paginate(iter, |result| match result {
        Ok(txn) => fmt.render(&txn),
        Err(err) => res = Err(err.into()),
    });
    let _ = fmt.stop();
    res
}

pub fn effects(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let id = matches.value_of("ID").expect("ID is required");
    let endpoint = account::Effects::new(id);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple);
    pager.paginate(iter, |result| match result {
        Ok(effect) => fmt.render(&effect),
        Err(err) => res = Err(err.into()),
    });
    let _ = fmt.stop();
    res
}

pub fn offers(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let id = matches.value_of("ID").expect("ID is required");
    let endpoint = account::Offers::new(id);
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple);
    pager.paginate(iter, |result| match result {
        Ok(offer) => fmt.render(&offer),
        Err(err) => res = Err(err.into()),
    });
    let _ = fmt.stop();
    res
}
