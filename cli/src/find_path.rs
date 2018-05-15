use super::pager::Pager;
use asset_identifier;
use clap::ArgMatches;
use error::Result;
use fmt::{Formatter, Simple};
use stellar_client::{endpoint::payment, resources::Amount, sync, sync::Client};

pub fn find_path(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = {
        let destination_account = matches
            .value_of("to")
            .expect("Destination account is a required field");
        let source_account = matches
            .value_of("from")
            .expect("Source account is a required field");
        let destination_amount: Amount = matches
            .value_of("amount")
            .expect("Destination amount is a required field")
            .parse()
            .expect("Amount must be properly formatted");
        let destination_asset_str = matches
            .value_of("asset")
            .expect("Destination asset is a required field");
        let destination_asset = asset_identifier::from_str(destination_asset_str)?;
        let endpoint = payment::FindPath::new(
            source_account,
            destination_account,
            destination_asset,
            destination_amount,
        );

        endpoint
    };

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());

    let mut fmt = Formatter::start_stdout(Simple::new());
    pager.paginate(iter, |result| match result {
        Ok(txn) => fmt.render(&txn),
        Err(err) => res = Err(err.into()),
    });
    let _ = fmt.stop();
    res
}
