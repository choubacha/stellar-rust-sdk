#![deny(warnings)]
//! A basic CLI for interactions with the stellar network.
extern crate chrono;
extern crate clap;
extern crate stellar_client;

use clap::{App, AppSettings, Arg, SubCommand};
use pager::Pager;
use stellar_client::{error::Error, sync::Client};
use error::CliError;

mod account;
mod assets;
mod cursor;
mod error;
mod fmt;
mod ledgers;
mod ordering;
mod pager;
mod resolution;
mod trades;
mod transactions;

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
                .subcommand(
                    listable!(
                        SubCommand::with_name("operations")
                            .about("Fetch all operations part of a given transaction")
                            .arg(
                                Arg::with_name("Hash")
                                    .required(true)
                                    .help("The transaction hash for which to fetch operations")
                            )
                    )
                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("effects")
                            .about("Fetch effects that resulted from a given transaction")
                            .arg(
                                Arg::with_name("Hash")
                                    .required(true)
                                    .help("The transaction hash for which to fetch effects")
                            )
                    )
                ),
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
        .subcommand(
            SubCommand::with_name("ledgers")
                .about("Access lists of ledgers")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    listable!(
                        SubCommand::with_name("all")
                            .about("Fetch all ledgers")
                    ),
                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("payments")
                            .about("Fetch payment operations for a given ledger")
                            .arg(
                                Arg::with_name("sequence")
                                    .required(true)
                                    .help("The sequence of the ledger for which to fetch payments")
                            )
                    )
                )
        )
        .subcommand(
            SubCommand::with_name("trades")
                .about("Access lists of trades")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    listable!(
                        SubCommand::with_name("all")
                            .about("Fetch all trades")
                            .arg(
                                Arg::with_name("base_asset_type")
                                    .long("base_asset_type")
                                    .takes_value(true)
                                    .help("Filters trades with a base_asset_type"),
                            )
                            .arg(
                                Arg::with_name("base_asset_code")
                                    .long("base_asset_code")
                                    .takes_value(true)
                                    .help("Filters trades with a base_asset_code.  Not required for XLM"),
                            )
                            .arg(
                                Arg::with_name("base_asset_issuer")
                                    .long("base_asset_issuer")
                                    .takes_value(true)
                                    .help("Filters trades with a base_asset_issuer.  Not required for XLM"),
                            )
                            .arg(
                                Arg::with_name("counter_asset_type")
                                    .long("counter_asset_type")
                                    .takes_value(true)
                                    .help("Filters trades with a counter_asset_type"),
                            )
                            .arg(
                                Arg::with_name("counter_asset_code")
                                    .long("counter_asset_code")
                                    .takes_value(true)
                                    .help("Filters trades with a counter_asset_code.  Not required for XLM"),
                            )
                            .arg(
                                Arg::with_name("counter_asset_issuer")
                                    .long("counter_asset_issuer")
                                    .takes_value(true)
                                    .help("Filters trades with a counter_asset_issuer.  Not required for XLM"),
                            )
                            .arg(
                                Arg::with_name("offer_id")
                                    .long("offer_id")
                                    .takes_value(true)
                                    .help("Filters trades that are a part of a particular offer_id"),
                            )
                    ),
                )
                .subcommand(
                    SubCommand::with_name("aggregations")
                        .about("Fetch aggregate statistics over a specified time range.")
                        .arg(
                            Arg::with_name("start_time")
                                .long("start_time")
                                .takes_value(true)
                                .required(true)
                                .help("Lower time boundary in ISO 8601 format, ex: 2017-11-28T12:00:09Z"),
                        )
                        .arg(
                            Arg::with_name("end_time")
                                .long("end_time")
                                .takes_value(true)
                                .required(true)
                                .help("Upper time boundary in ISO 8601 format, ex: 2017-11-28T12:00:09Z"),
                        )
                        .arg(
                            Arg::with_name("resolution")
                                .long("resolution")
                                .takes_value(true)
                                .required(true)
                                .help("Segment duration in format <number><unit> where units are s, m, h, d.  ie: 10h == 10 hours"),
                        )
                        .arg(
                            Arg::with_name("base_asset_type")
                                .long("base_asset_type")
                                .takes_value(true)
                                .required(true)
                                .help("Filters trades with a base_asset_type"),
                        )
                        .arg(
                            Arg::with_name("base_asset_code")
                                .long("base_asset_code")
                                .takes_value(true)
                                .help("Filters trades with a base_asset_code.  Not required for XLM"),
                        )
                        .arg(
                            Arg::with_name("base_asset_issuer")
                                .long("base_asset_issuer")
                                .takes_value(true)
                                .help("Filters trades with a base_asset_issuer.  Not required for XLM"),
                        )
                        .arg(
                            Arg::with_name("counter_asset_type")
                                .long("counter_asset_type")
                                .takes_value(true)
                                .required(true)
                                .help("Filters trades with a counter_asset_type"),
                        )
                        .arg(
                            Arg::with_name("counter_asset_code")
                                .long("counter_asset_code")
                                .takes_value(true)
                                .help("Filters trades with a counter_asset_code.  Not required for XLM"),
                        )
                        .arg(
                            Arg::with_name("counter_asset_issuer")
                                .long("counter_asset_issuer")
                                .takes_value(true)
                                .help("Filters trades with a counter_asset_issuer.  Not required for XLM"),
                        )
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
            ("operations", Some(sub_m)) => transactions::operations(&client, sub_m),
            ("payments", Some(sub_m)) => transactions::payments(&client, sub_m),
            ("effects", Some(sub_m)) => transactions::effects(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("assets", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(sub_m)) => assets::all(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("ledgers", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(sub_m)) => ledgers::all(&client, sub_m),
            ("payments", Some(sub_m)) => ledgers::payments(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("trades", Some(sub_m)) => match sub_m.subcommand() {
            ("aggregations", Some(sub_m)) => trades::aggregations(&client, sub_m),
            ("all", Some(sub_m)) => trades::all(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        _ => return print_help_and_exit(),
    };

    match result {
        Ok(_) => {}
        Err(CliError::ClientError(Error::BadResponse(err))) => {
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
