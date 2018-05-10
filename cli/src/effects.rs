use super::{cursor, ordering, pager::Pager};
use clap::ArgMatches;
use error::Result;
use fmt::{Formatter, Simple};
use stellar_client::{endpoint::effect, sync::{self, Client}};

/// Using a client and the arguments from the command line, iterates over the results
/// and displays them to the end user.
pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = effect::All::default();
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
    res
}
