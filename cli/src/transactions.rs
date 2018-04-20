use stellar_resources::OperationKind;
use stellar_client::{endpoint::transaction, sync::{self, Client}};
use clap::ArgMatches;
use super::{cursor, ordering, pager::Pager};
use error::Result;

pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = transaction::All::default();
    let endpoint = pager.assign(endpoint);
    let endpoint = cursor::assign_from_arg(matches, endpoint);
    let endpoint = ordering::assign_from_arg(matches, endpoint);

    let iter = sync::Iter::new(&client, endpoint);

    let mut res = Ok(());
    pager.paginate(iter, |result| match result {
        Ok(txn) => {
            println!("ID:             {}", txn.id());
            println!("source account: {}", txn.source_account());
            println!("created at:     {}", txn.created_at());
            println!();
        }
        Err(err) => res = Err(err.into()),
    });
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
    pager.paginate(iter, |result| match result {
        Ok(op) => {
            println!("ID:           {}", op.id());
            if let &OperationKind::Payment(ref pymt) = op.kind() {
                println!("To account:   {}", pymt.to());
                println!("From account: {}", pymt.from());
                println!("Asset");
                println!("  Type:       {}", pymt.asset().asset_type());
                println!("  Code:       {}", pymt.asset().code());
                println!("  Issuer:     {}", pymt.asset().issuer());
                println!("Amount:       {}", pymt.amount());
            } else {
                panic!("Did not receive Payment operation");
            }
            println!();
        }
        Err(err) => res = Err(err.into()),
    });
    res
}

pub fn details(client: &Client, matches: &ArgMatches) -> Result<()> {
    let hash = matches
        .value_of("HASH")
        .expect("Transaction identifier hash is required");
    let endpoint = transaction::Details::new(&hash);
    let txn = client.request(endpoint)?;

    println!("Hash:                    {}", txn.hash());
    println!("ledger:                  {}", txn.ledger());
    println!("created at:              {}", txn.created_at());
    println!("source account:          {}", txn.source_account());
    println!("source account sequence: {}", txn.source_account_sequence());
    println!("fee paid:                {}", txn.fee_as_amount());
    println!("operation count:         {}", txn.operation_count());
    println!();

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
    pager.paginate(iter, |result| match result {
        Ok(effect) => {
            println!("ID:   {}", effect.id());
            println!("Type: {}", effect.kind_name());
            println!();
        }
        Err(err) => res = Err(err.into()),
    });
    res
}
