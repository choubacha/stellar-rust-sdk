extern crate clap;
extern crate stellar_client;
extern crate stellar_resources;

use clap::{App, Arg, SubCommand};
use stellar_client::sync::Client;
use stellar_client::endpoint::account;

fn main() {
    let matches = App::new("Stellar CLI")
        .version("0.1")
        .about("Access the stellar horizon API via the command line.")
        .subcommand(
            SubCommand::with_name("account")
                .about("Access information about accounts or related to them")
                .subcommand(
                    SubCommand::with_name("details")
                        .about("Fetch details about a specific account")
                        .arg(
                            Arg::with_name("ID")
                                .required(true)
                                .help("The identifier of the account to look up"),
                        ),
                ),
        )
        .get_matches();

    let client = Client::horizon_test().unwrap();
    match matches.subcommand() {
        ("account", Some(sub_m)) => match sub_m.subcommand() {
            ("details", Some(sub_m)) => {
                let id = sub_m.value_of("ID").expect("ID is required");
                let endpoint = account::Details::new(id);
                if let Ok(account) = client.request(endpoint) {
                    println!("ID:       {}", account.id());
                    println!("Sequence: {}", account.sequence());
                } else {
                    eprintln!("Could not find account with id {}", id);
                    ::std::process::exit(1);
                }
            }
            _ => {
                eprintln!("No action specified. Try using --help to see available actions.");
                ::std::process::exit(1);
            }
        },
        _ => {
            eprintln!("No action specified. Try using --help to see available actions.");
            ::std::process::exit(1);
        }
    };
}
