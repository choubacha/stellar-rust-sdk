extern crate clap;
extern crate stellar_client;
extern crate stellar_resources;

use clap::{App, AppSettings, Arg, SubCommand};
use stellar_client::sync::Client;

fn build_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Stellar CLI")
        .version("0.1")
        .about("Access the stellar horizon API via the command line.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("account")
                .about("Access information about accounts or related to them")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    SubCommand::with_name("details")
                        .about("Fetch details about a specific account")
                        .arg(
                            Arg::with_name("ID")
                                .required(true)
                                .help("The identifier of the account to look up"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("transactions")
                        .about("Fetch transactions for an account")
                        .arg(
                            Arg::with_name("ID")
                                .required(true)
                                .help("The identifier of the account to look up"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("transactions")
                .about("Access lists of transactions")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(SubCommand::with_name("all").about("Fetch all transactions")),
        )
}

fn print_help_and_exit() {
    build_app().print_help().expect("Error printing help");
    println!();
    ::std::process::exit(1);
}

fn next_page() -> bool {
    println!("-- press q to quit --");
    let mut input = String::new();
    match ::std::io::stdin().read_line(&mut input) {
        Ok(_) => !input.starts_with("q"),
        _ => false,
    }
}

mod account {
    use clap::ArgMatches;
    use stellar_client::{endpoint::{account, Order}, error::Error, sync::Client};
    use super::next_page;

    pub fn details<'a>(client: Client, matches: &'a ArgMatches) {
        let id = matches.value_of("ID").expect("ID is required");
        let endpoint = account::Details::new(id);
        match client.request(endpoint) {
            Ok(account) => {
                println!("ID:       {}", account.id());
                println!("Sequence: {}", account.sequence());
            }
            Err(Error::BadResponse(err)) => {
                eprintln!("{}", err);
                ::std::process::exit(1);
            }
            err => {
                eprintln!("An unknown error occurred: {:?}", err);
                ::std::process::exit(1);
            }
        }
    }

    pub fn transactions<'a>(client: Client, matches: &'a ArgMatches, cursor: Option<String>) {
        let id = matches.value_of("ID").expect("ID is required");
        let mut endpoint = account::Transactions::new(id).order(Order::Desc);

        if let Some(c) = cursor {
            endpoint = endpoint.cursor(&c);
        }

        match client.request(endpoint) {
            Ok(records) => {
                for txn in records.records().iter() {
                    println!("ID:         {}", txn.id());
                    println!("account id: {}", txn.source_account());
                    println!("created at: {}", txn.created_at());
                    println!("");
                }
                if records.records().len() > 0 && next_page() {
                    transactions(client, &matches, Some(records.next_cursor().to_string()))
                }
            }
            Err(Error::BadResponse(err)) => {
                eprintln!("{}", err);
                ::std::process::exit(1);
            }
            err => {
                eprintln!("An unknown error occurred: {:?}", err);
                ::std::process::exit(1);
            }
        }
    }
}

mod transactions {
    use stellar_client::{endpoint::{transaction, Order}, error::Error, sync::Client};
    use super::next_page;

    pub fn all<'a>(client: Client, cursor: Option<String>) {
        let mut endpoint = transaction::All::default().order(Order::Desc);

        if let Some(c) = cursor {
            endpoint = endpoint.cursor(&c);
        }

        match client.request(endpoint) {
            Ok(records) => {
                for txn in records.records().iter() {
                    println!("ID:         {}", txn.id());
                    println!("account id: {}", txn.source_account());
                    println!("created at: {}", txn.created_at());
                    println!("");
                }
                if records.records().len() > 0 && next_page() {
                    all(client, Some(records.next_cursor().to_string()))
                }
            }
            Err(Error::BadResponse(err)) => {
                eprintln!("{}", err);
                ::std::process::exit(1);
            }
            err => {
                eprintln!("An unknown error occurred: {:?}", err);
                ::std::process::exit(1);
            }
        }
    }
}

fn main() {
    let matches = build_app().get_matches();

    let client = Client::horizon_test().unwrap();
    match matches.subcommand() {
        ("account", Some(sub_m)) => match sub_m.subcommand() {
            ("details", Some(sub_m)) => account::details(client, sub_m),
            ("transactions", Some(sub_m)) => account::transactions(client, sub_m, None),
            _ => print_help_and_exit(),
        },
        ("transactions", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(_)) => transactions::all(client, None),
            _ => print_help_and_exit(),
        },
        _ => print_help_and_exit(),
    };
}
