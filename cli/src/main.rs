#![deny(warnings)]
//! A basic CLI for interactions with the stellar network.
extern crate clap;
extern crate stellar_client;
extern crate stellar_resources;

use clap::{App, AppSettings, Arg, SubCommand};
use stellar_client::{error::Error, sync::Client};

mod pager;
mod ordering;
mod cursor;
use pager::Pager;

fn build_app<'a, 'b>() -> App<'a, 'b> {
    macro_rules! listable {
        ($e:expr) => {
            Pager::add(
                ordering::add(
                    cursor::add($e)
                )
            )
        }
    }

    App::new("Stellar CLI")
        .version("0.1")
        .about("Access the stellar horizon API via the command line.")
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("host")
                .takes_value(true)
                .short("h")
                .long("host")
                .conflicts_with_all(&["test-net", "pub-net"])
                .help("The host url to attempt to connect to. If not specified will default to test-net."),
        )
        .arg(
            Arg::with_name("test-net")
                .long("test-net")
                .conflicts_with_all(&["host", "pub-net"])
                .help("Connects to the test net. This is the default"),
        )
        .arg(
            Arg::with_name("pub-net")
                .long("pub-net")
                .conflicts_with_all(&["host", "test-net"])
                .help("Connects to the public net."),
        )
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
                    listable!(
                        SubCommand::with_name("transactions")
                            .about("Fetch transactions for an account")
                            .arg(
                                Arg::with_name("ID")
                                    .required(true)
                                    .help("The identifier of the account to look up"),
                            )
                    ),

                ),
        )
        .subcommand(
            SubCommand::with_name("transactions")
                .about("Access lists of transactions")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    listable!(
                        SubCommand::with_name("all")
                            .about("Fetch all transactions")
                    )
                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("payments")
                            .about("Fetch payment operations part of a given transaction")
                            .arg(
                                Arg::with_name("Hash")
                                    .required(true)
                                    .help("The transaction hash for which to fetch payments")
                            )
                    )
                )
                .subcommand(
                    SubCommand::with_name("details")
                        .about("Fetch details about a specific transaction")
                        .arg(
                            Arg::with_name("HASH")
                                .required(true)
                                .help("The identifier of the transaction to look up"),
                        ),
                )
        )
        .subcommand(
            SubCommand::with_name("assets")
                .about("Access lists of assets")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    listable!(
                        SubCommand::with_name("all")
                            .about("Fetch all assets")
                            .arg(
                                Arg::with_name("code")
                                    .short("c")
                                    .long("code")
                                    .takes_value(true)
                                    .help("Filters the set by a particular asset code"),
                            )
                            .arg(
                                Arg::with_name("issuer")
                                    .short("i")
                                    .long("issuer")
                                    .takes_value(true)
                                    .help("Filters the set by a particular asset issuer"),
                            )
                    ),
                ),
        )
}

fn main() {
    let matches = build_app().get_matches();

    let client = if let Some(host) = matches.value_of("host") {
        Client::new(&host).expect("Failed to initialize client")
    } else if matches.is_present("pub-net") {
        Client::horizon().unwrap()
    } else {
        Client::horizon_test().unwrap()
    };

    // Master match block. All subcommands need to be captured here.
    let result = match matches.subcommand() {
        ("account", Some(sub_m)) => match sub_m.subcommand() {
            ("details", Some(sub_m)) => account::details(&client, sub_m),
            ("transactions", Some(sub_m)) => account::transactions(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("transactions", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(sub_m)) => transactions::all(&client, sub_m),
            ("details", Some(sub_m)) => transactions::details(&client, sub_m),
            ("payments", Some(sub_m)) => transactions::payments(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("assets", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(m)) => assets::all(&client, m),
            _ => return print_help_and_exit(),
        },
        _ => return print_help_and_exit(),
    };

    match result {
        Ok(_) => {}
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

fn print_help_and_exit() {
    build_app().print_help().expect("Error printing help");
    println!();
    ::std::process::exit(1);
}

mod account;
mod transactions;
mod assets;
