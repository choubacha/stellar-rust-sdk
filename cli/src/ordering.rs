use clap::{App, Arg, ArgMatches};
use stellar_client::endpoint::{Direction::{Asc, Desc}, Order};

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
pub fn assign_from_arg<T>(arg: &ArgMatches, order: T) -> T
where
    T: Order,
{
    order.with_order(match arg.value_of(ARG_NAME) {
        Some(s) if s == ASC => Asc,
        Some(s) if s == DESC => Desc,
        _ => Desc,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use stellar_client::endpoint::Direction;

    struct Foo {
        order: Option<Direction>,
    }

    impl Order for Foo {
        fn with_order(mut self, order: Direction) -> Foo {
            self.order = Some(order);
            self
        }

        fn order(&self) -> Option<Direction> {
            self.order
        }
    }

    fn test_app<'a, 'b>() -> App<'a, 'b> {
        add(App::new("test"))
    }

    fn get_matches(args: Vec<&str>) -> ArgMatches {
        let app = test_app();
        app.get_matches_from(args)
    }

    #[test]
    fn it_sets_desc() {
        let arg_matches = get_matches(vec!["test", "--order", "desc"]);
        let foo = Foo { order: None };
        let foo = assign_from_arg(&arg_matches, foo);
        assert_eq!(foo.order(), Some(Desc));
    }

    #[test]
    fn it_sets_asc() {
        let arg_matches = get_matches(vec!["test", "--order", "asc"]);
        let foo = Foo { order: None };
        let foo = assign_from_arg(&arg_matches, foo);
        assert_eq!(foo.order(), Some(Asc));
    }

    #[test]
    fn it_defaults_to_desc() {
        let arg_matches = get_matches(vec!["test"]);
        let foo = Foo { order: None };
        let foo = assign_from_arg(&arg_matches, foo);
        assert_eq!(foo.order(), Some(Desc));
    }
}
