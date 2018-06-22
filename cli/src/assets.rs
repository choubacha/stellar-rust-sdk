use super::{cursor, ordering, pager::Pager};
use clap::ArgMatches;
use error::Result;
use fmt::{Formatter, Simple};
use stellar_client::{
    endpoint::asset, sync::{self, Client},
};

/// Using a client and the arguments from the command line, iterates over the results
/// and displays them to the end user.
pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = {
        let mut endpoint = asset::All::default();

        if let Some(code) = matches.value_of("code") {
            endpoint = endpoint.with_asset_code(code);
        }
        if let Some(issuer) = matches.value_of("issuer") {
            endpoint = endpoint.with_asset_issuer(issuer);
        }

        endpoint = pager.assign(endpoint);
        endpoint = ordering::assign_from_arg(matches, endpoint);
        cursor::assign_from_arg(matches, endpoint)
    };
    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    let mut fmt = Formatter::start_stdout(Simple::new());
    pager.paginate(iter, |result| match result {
        Ok(asset) => fmt.render(&asset),
        Err(err) => res = Err(err.into()),
    });
    let _ = fmt.stop();
    res
}
