use clap::{App, Arg, ArgMatches};
use stellar_client::endpoint::{Order, Order::Asc, Order::Desc};

static ARG_NAME: &'static str = "order";
static ASC: &'static str = "asc";
static DESC: &'static str = "desc";

/// Appends the order arg to the app and returns a newly owned app.
pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name(ARG_NAME)
            .long("order")
            .short("o")
            .takes_value(true)
            .default_value(DESC)
            .possible_values(&[ASC, DESC])
            .help("Specify the order to return the results"),
    )
}

/// Parses the argument matches and returns the order to use.
pub fn from_arg<'a>(arg: &'a ArgMatches) -> Order {
    match arg.value_of(ARG_NAME) {
        Some(s) if s == ASC => Asc,
        Some(s) if s == DESC => Desc,
        _ => Desc,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app<'a, 'b>() -> App<'a, 'b> {
        add(App::new("test"))
    }

    fn get_matches(args: Vec<&str>) -> ArgMatches {
        let app = test_app();
        app.get_matches_from(args)
    }

    #[test]
    fn it_sets_desc() {
        let order = from_arg(&get_matches(vec!["test", "--order", "desc"]));
        assert_eq!(order, Desc);
    }

    #[test]
    fn it_sets_asc() {
        let order = from_arg(&get_matches(vec!["test", "--order", "asc"]));
        assert_eq!(order, Asc);
    }

    #[test]
    fn it_defaults_to_desc() {
        let order = from_arg(&get_matches(vec!["test"]));
        assert_eq!(order, Desc);
    }
}
