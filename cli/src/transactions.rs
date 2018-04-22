use stellar_client::{endpoint::transaction, resources::OperationKind, sync::{self, Client}};
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
            println!("ID:             {}", op.id());
            match op.kind() {
                &OperationKind::CreateAccount(ref create_account) => {
                    println!("Operation Kind:   Create Account");
                    println!("Account:          {}", create_account.account());
                    println!("Funder:           {}", create_account.funder());
                    println!("Starting Balance: {}", create_account.starting_balance());
                }
                &OperationKind::Payment(ref payment) => {
                    println!("Operation Kind: Payment");
                    println!("To account:     {}", payment.to());
                    println!("From account:   {}", payment.from());
                    println!("Asset Type:     {}", payment.asset().asset_type());
                    println!("Asset Code:     {}", payment.asset().code());
                    println!("Asset Issuer:   {}", payment.asset().issuer());
                    println!("Amount:         {}", payment.amount());
                }
                &OperationKind::PathPayment(ref path_payment) => {
                    println!("Operation Kind:           Path Payment");
                    println!("To account:               {}", path_payment.to());
                    println!("From account:             {}", path_payment.from());
                    println!(
                        "Source Asset Type:        {}",
                        path_payment.source_asset().asset_type()
                    );
                    println!(
                        "Source Asset Code:        {}",
                        path_payment.source_asset().code()
                    );
                    println!(
                        "Source Asset Issuer:      {}",
                        path_payment.source_asset().issuer()
                    );
                    println!("Source Amount:            {}", path_payment.source_amount());
                    println!(
                        "Destination Asset Type:   {}",
                        path_payment.destination_asset().asset_type()
                    );
                    println!(
                        "Destination Asset Code:   {}",
                        path_payment.destination_asset().code()
                    );
                    println!(
                        "Destination Asset Issuer: {}",
                        path_payment.destination_asset().issuer()
                    );
                    println!(
                        "Destination Amount:       {}",
                        path_payment.destination_amount()
                    );
                }
                _ => println!("Something unexpected happened"),
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
