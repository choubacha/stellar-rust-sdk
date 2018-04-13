use clap::{App, Arg, ArgMatches};
use std::str::FromStr;
use stellar_client::endpoint::Limit;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pager {
    size: PageSize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PageSize {
    All,
    Size(usize),
}

impl Pager {
    pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.arg(
            Arg::with_name("all")
                .long("all")
                .conflicts_with("page-size")
                .help("Query for all records"),
        ).arg(
            Arg::with_name("page-size")
                .long("page-by")
                .conflicts_with("all")
                .takes_value(true)
                .validator(Self::validator)
                .help("Query and display in a specific page size. Defaults to 10"),
        )
    }

    fn validator(v: String) -> Result<(), String> {
        usize::from_str(&v)
            .map(|_| ())
            .map_err(|_| String::from("Page size must be a positive integer"))
    }

    pub fn from_arg(arg: &ArgMatches) -> Pager {
        if let Some(size) = arg.value_of("page-size") {
            Pager {
                size: PageSize::Size(usize::from_str(&size).unwrap_or(10)),
            }
        } else if arg.is_present("all") {
            Pager {
                size: PageSize::All,
            }
        } else {
            Pager {
                size: PageSize::Size(10),
            }
        }
    }

    pub fn paginate<I, T, F>(&self, iter: I, mut render: F)
    where
        I: Iterator<Item = T>,
        F: FnMut(T),
    {
        match self.size {
            PageSize::All => iter.for_each(render),
            PageSize::Size(page_size) => for (index, item) in iter.enumerate() {
                render(item);

                if (index + 1) % page_size == 0 && !Self::next_page() {
                    break;
                }
            },
        }
    }

    pub fn assign<T>(&self, limit: T) -> T
    where
        T: Limit,
    {
        limit.with_limit(self.horizon_page_limit())
    }

    fn horizon_page_limit(&self) -> u32 {
        const MAX: usize = 200;
        match self.size {
            PageSize::All => MAX as u32,
            PageSize::Size(size) => {
                if size > MAX {
                    MAX as u32
                } else {
                    size as u32
                }
            }
        }
    }

    fn next_page() -> bool {
        println!("-- press q to quit --");
        let mut input = String::new();
        match ::std::io::stdin().read_line(&mut input) {
            Ok(_) => !input.starts_with('q'),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app<'a, 'b>() -> App<'a, 'b> {
        Pager::add(App::new("test"))
    }

    fn get_matches(args: Vec<&str>) -> ArgMatches {
        let app = test_app();
        app.get_matches_from(args)
    }

    #[test]
    fn it_defaults_to_page_size_ten() {
        let pager = Pager::from_arg(&get_matches(vec!["test"]));
        assert_eq!(pager.size, PageSize::Size(10));
        assert_eq!(pager.horizon_page_limit(), 10);
    }

    #[test]
    fn it_can_change_the_size() {
        let pager = Pager::from_arg(&get_matches(vec!["test", "--page-by", "15"]));
        assert_eq!(pager.size, PageSize::Size(15));
        assert_eq!(pager.horizon_page_limit(), 15);
    }

    #[test]
    fn it_can_keeps_the_limit_small_if_size_is_large() {
        let pager = Pager::from_arg(&get_matches(vec!["test", "--page-by", "1000"]));
        assert_eq!(pager.size, PageSize::Size(1000));
        assert_eq!(pager.horizon_page_limit(), 200);
    }

    #[test]
    fn it_can_switch_to_all() {
        let pager = Pager::from_arg(&get_matches(vec!["test", "--all"]));
        assert_eq!(pager.size, PageSize::All);
        assert_eq!(pager.horizon_page_limit(), 200);
    }

    #[test]
    fn it_can_assign_a_limit() {
        struct Foo {
            limit: Option<u32>,
        }

        impl Limit for Foo {
            fn with_limit(mut self, limit: u32) -> Foo {
                self.limit = Some(limit);
                self
            }

            fn limit(&self) -> Option<u32> {
                self.limit
            }
        }

        let pager = Pager::from_arg(&get_matches(vec!["test", "--page-by", "15"]));
        let limit = pager.assign(Foo { limit: None });
        assert_eq!(limit.limit(), Some(15));
    }
}
