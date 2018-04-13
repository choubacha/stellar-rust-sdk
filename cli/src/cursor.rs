use clap::{App, Arg, ArgMatches};
use stellar_client::endpoint::Cursor;

static ARG_NAME: &'static str = "cursor";

/// Appends the cursor arg to the app and returns a newly owned app.
pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .takes_value(true)
            .help("Specify the cursor of the oldest record to return."),
    )
}

/// Parses the argument matches and returns the cursor with the option
/// attached to it.
pub fn assign_from_arg<C>(arg: &ArgMatches, cursor: C) -> C
where
    C: Cursor,
{
    match arg.value_of(ARG_NAME) {
        Some(cur) => cursor.with_cursor(cur),
        None => cursor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Foo {
        cursor: Option<String>,
    }

    impl Cursor for Foo {
        fn with_cursor(mut self, cursor: &str) -> Foo {
            self.cursor = Some(cursor.to_owned());
            self
        }

        fn cursor(&self) -> Option<&str> {
            self.cursor.as_ref().map(|s| &**s)
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
    fn it_sets_the_cursor_if_provided() {
        let arg_matches = get_matches(vec!["test", "--cursor", "123abc"]);
        let cursor = Foo { cursor: None };
        let cursor = assign_from_arg(&arg_matches, cursor);
        assert_eq!(cursor.cursor(), Some("123abc"));
    }

    #[test]
    fn it_defaults_to_none() {
        let arg_matches = get_matches(vec!["test"]);
        let cursor = Foo { cursor: None };
        let cursor = assign_from_arg(&arg_matches, cursor);
        assert_eq!(cursor.cursor(), None);
    }
}
