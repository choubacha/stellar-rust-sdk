#![deny(warnings)]
//! A basic CLI for interactions with the stellar network.
extern crate chrono;
extern crate clap;
extern crate stellar_client;

use clap::{App, AppSettings, Arg, SubCommand};
use error::CliError;
use pager::Pager;
use stellar_client::{error::Error, sync::Client};

mod account;
mod assets;
mod cursor;
mod effects;
mod error;
mod find_path;
mod fmt;
mod ledgers;
mod operations;
mod orderbook;
mod ordering;
mod pager;
mod payments;
mod resolution;
mod trades;
mod transactions;

fn build_app<'a, 'b>() -> App<'a, 'b> {
    macro_rules! listable {
        ($e:expr) => {
            Pager::add(ordering::add(cursor::add($e)))
        };
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
                    SubCommand::with_name("data")
                        .about("Fetch and account's metadata for a particular key")
                        .arg(
                            Arg::with_name("ID")
                                .required(true)
                                .help("The identifier of the account to look up"),
                        )
                        .arg(
                            Arg::with_name("key")
                                .required(true)
                                .help("The key for the metadata you wish to fetch"),
                        ),
                )
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
                        SubCommand::with_name("trades")
                            .about("Fetch trades for an account")
                            .arg(
                                Arg::with_name("ID")
                                    .required(true)
                                    .help("The identifier of the account to look up"),
                            )
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

                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("effects")
                            .about("Fetch all effects for an account")
                            .arg(
                                Arg::with_name("ID")
                                    .required(true)
                                    .help("The identifier of the account to look up"),
                            )
                    ),

                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("offers")
                            .about("Fetch all open offers for an account")
                            .arg(
                                Arg::with_name("ID")
                                    .required(true)
                                    .help("The identifier of the account to look up"),
                            )
                    ),

                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("operations")
                            .about("Fetch all operations associated with an account")
                            .arg(
                                Arg::with_name("ID")
                                    .required(true)
                                    .help("The identifier of the account to look up"),
                            )
                    ),

                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("payments")
                            .about("Fetch all payments associated with an account")
                            .arg(
                                Arg::with_name("ID")
                                    .required(true)
                                    .help("The identifier of the account to look up"),
                            )
                    ),

                ),
        )
        .subcommand(
            SubCommand::with_name("effects")
                .about("Access lists of effects")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    listable!(
                        SubCommand::with_name("all")
                            .about("Fetch all effects")
                    )
                )
        )
        .subcommand(
            SubCommand::with_name("payments")
                .about("Access lists of payments")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    listable!(
                        SubCommand::with_name("all")
                            .about("Fetch all payments")
                    )
                ),
        )
        .subcommand(
            SubCommand::with_name("find-path")
                .about("Fetch possible payment paths")
                .arg(
                    Arg::with_name("to")
                        .long("to")
                        .takes_value(true)
                        .required(true)
                        .help("The destination account id of the payment path to query"),
                )
                .arg(
                    Arg::with_name("from")
                        .long("from")
                        .takes_value(true)
                        .required(true)
                        .help("The source account id of the payment path to query"),
                )
                .arg(
                    Arg::with_name("asset")
                        .long("asset")
                        .takes_value(true)
                        .required(true)
                        .help("Specifies destination asset for find-path to return. format:  <asset_code>-<asset_issuer>, or xlm if lumens"),
                )
                .arg(
                    Arg::with_name("amount")
                        .long("amount")
                        .takes_value(true)
                        .required(true)
                        .help("The amount of the destination asset resulting from the payment path"),
                )
        )
        .subcommand(
            SubCommand::with_name("operations")
                .about("Access lists of operations")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    listable!(
                        SubCommand::with_name("all")
                            .about("Fetch all operations")
                    )
                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("effects")
                            .about("Fetch the effects from a specific operation.")
                            .arg(
                                Arg::with_name("ID")
                                    .required(true)
                                    .help("The ID of the operation")
                            )
                    )
                )
        )
        .subcommand(
            SubCommand::with_name("orderbook")
                .about("Access information about a collection of offers for a asset pair")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    SubCommand::with_name("details")
                        .about("Fetch details about bids and asks for a given asset pair")
                        .arg(
                            Arg::with_name("base")
                                .long("base")
                                .takes_value(true)
                                .required(true)
                                .help("Specifies base_asset for orderbook to return. format:  <asset_code>-<asset_issuer>, or xlm if lumens"),
                        )
                        .arg(
                            Arg::with_name("counter")
                                .long("counter")
                                .takes_value(true)
                                .required(true)
                                .help("Specifies counter_asset for orderbook to return. format:  <asset_code>-<asset_issuer>, or xlm if lumens"),
                        )
                        .arg(
                            Arg::with_name("limit")
                                .long("limit")
                                .takes_value(true)
                                .help("Maximum number of bids and asks to return, defaults to 20"),
                        )
                )
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
                    SubCommand::with_name("details")
                        .about("Fetch details for a given ledger")
                        .arg(
                            Arg::with_name("sequence")
                                .required(true)
                                .help("The sequence of the ledger for which to fetch details")
                        )
                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("effects")
                            .about("Fetch effects for a given ledger")
                            .arg(
                                Arg::with_name("sequence")
                                    .required(true)
                                    .help("The sequence of the ledger for which to fetch effects")
                            )
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
                .subcommand(
                    listable!(
                        SubCommand::with_name("operations")
                            .about("Fetch operations for a given ledger")
                            .arg(
                                Arg::with_name("sequence")
                                    .required(true)
                                    .help("The sequence of the ledger for which to fetch operations")
                            )
                    )
                )
                .subcommand(
                    listable!(
                        SubCommand::with_name("transactions")
                            .about("Fetch transactions for a given ledger")
                            .arg(
                                Arg::with_name("sequence")
                                    .required(true)
                                    .help("The sequence of the ledger for which to fetch transactions")
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
                                Arg::with_name("base")
                                    .long("base")
                                    .takes_value(true)
                                    .required(true)
                                    .help("Filters trades with a given base_asset. format:  <asset_code>-<asset_issuer>, or xlm if lumens"),
                            )
                            .arg(
                                Arg::with_name("counter")
                                    .long("counter")
                                    .takes_value(true)
                                    .required(true)
                                    .help("Filters trades with a given counter_asset. format:  <asset_code>-<asset_issuer>, or xlm if lumens"),
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
                                .possible_values(&["1m", "5m", "15m", "1h", "1d", "1w"])
                                .help("The resolution of each segment. Limited to a set of valid inputs")
                        )
                        .arg(
                            Arg::with_name("base")
                                .long("base")
                                .takes_value(true)
                                .required(true)
                                .help("Filters trades with a given base_asset. format:  <asset_code>-<asset_issuer>, or xlm if lumens"),
                        )
                        .arg(
                            Arg::with_name("counter")
                                .long("counter")
                                .takes_value(true)
                                .required(true)
                                .help("Filters trades with a given counter_asset. format:  <asset_code>-<asset_issuer>, or xlm if lumens"),
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
            ("data", Some(sub_m)) => account::data(&client, sub_m),
            ("details", Some(sub_m)) => account::details(&client, sub_m),
            ("trades", Some(sub_m)) => account::trades(&client, sub_m),
            ("transactions", Some(sub_m)) => account::transactions(&client, sub_m),
            ("effects", Some(sub_m)) => account::effects(&client, sub_m),
            ("offers", Some(sub_m)) => account::offers(&client, sub_m),
            ("operations", Some(sub_m)) => account::operations(&client, sub_m),
            ("payments", Some(sub_m)) => account::payments(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("assets", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(sub_m)) => assets::all(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("effects", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(sub_m)) => effects::all(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("ledgers", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(sub_m)) => ledgers::all(&client, sub_m),
            ("details", Some(sub_m)) => ledgers::details(&client, sub_m),
            ("effects", Some(sub_m)) => ledgers::effects(&client, sub_m),
            ("payments", Some(sub_m)) => ledgers::payments(&client, sub_m),
            ("operations", Some(sub_m)) => ledgers::operations(&client, sub_m),
            ("transactions", Some(sub_m)) => ledgers::transactions(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("operations", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(sub_m)) => operations::all(&client, sub_m),
            ("effects", Some(sub_m)) => operations::effects(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("orderbook", Some(sub_m)) => match sub_m.subcommand() {
            ("details", Some(sub_m)) => orderbook::details(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("find-path", Some(sub_m)) => find_path::find_path(&client, sub_m),
        ("payments", Some(sub_m)) => match sub_m.subcommand() {
            ("all", Some(sub_m)) => payments::all(&client, sub_m),
            _ => return print_help_and_exit(),
        },
        ("trades", Some(sub_m)) => match sub_m.subcommand() {
            ("aggregations", Some(sub_m)) => trades::aggregations(&client, sub_m),
            ("all", Some(sub_m)) => trades::all(&client, sub_m),
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
