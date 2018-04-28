use stellar_client::{endpoint::payment, sync::{self, Client}};
use clap::ArgMatches;
use error::Result;
use fmt::{Formatter, Simple};
use super::{cursor, ordering, pager::Pager};

pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = payment::All::default();
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
