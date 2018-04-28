use stellar_client::{endpoint::payment, resources::OperationKind, sync::{self, Client}};
use clap::ArgMatches;
use error::Result;
use super::{cursor, ordering, pager::Pager};

pub fn all(client: &Client, matches: &ArgMatches) -> Result<()> {
    let pager = Pager::from_arg(&matches);

    let endpoint = payment::All::default();
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
