#![macro_use]
macro_rules! append {
    ($buffer:expr, $($args:tt)*) => {
        $buffer.push_str(&format!($($args)*));
        $buffer.push_str("\n");
    }
}

macro_rules! nest {
    ($buffer:expr, $fmt:ident, $to_nest:expr) => {
        if let Some(render) = $fmt.render($to_nest) {
            let nested_render = $fmt.nest().align_str(&render);
            $buffer.push_str(&nested_render);
        }
    };
}

macro_rules! indent {
    ($buffer:expr, $fmt:ident, $($args:tt)*) => {
        let nested = $fmt.nest().align_str(&format!($($args)*));
        $buffer.push_str(&nested);
    }
}

const INDENT_SPACES: usize = 4;

pub struct Simple {
    nest_level: usize,
}

impl Simple {
    pub fn new() -> Self {
        Self { nest_level: 0 }
    }

    fn nest(&self) -> Self {
        Self {
            nest_level: self.nest_level + 1,
        }
    }

    fn align_str(&self, text: &str) -> String {
        if self.nest_level == 0 {
            String::from(text)
        } else {
            indenter::indent(text, self.nest_level * INDENT_SPACES)
        }
    }
}

// Adapted from crate `textwrap`
// https://docs.rs/textwrap/0.10.0/src/textwrap/lib.rs.html#870-880
mod indenter {
    pub fn indent(source: &str, spaces: usize) -> String {
        let prefix = " ".repeat(spaces);
        let mut result = String::new();
        for line in source.lines() {
            if line.chars().any(|c| !c.is_whitespace()) {
                result.push_str(&prefix);
                result.push_str(line);
            }
            result.push('\n');
        }
        result
    }
}

mod account;
mod asset;
mod datum;
mod effect;
mod ledger;
mod offer;
mod operation;
mod orderbook;
mod trade_aggregation;
mod transaction;

#[cfg(test)]
mod tests {

    #[test]
    fn it_appends_to_a_buffer() {
        let mut buffer = String::new();
        append!(buffer, "Fantastic Mr Fox");

        assert_eq!(buffer, "Fantastic Mr Fox\n".to_string());
    }

    #[test]
    fn it_appends_to_a_buffer_with_indentation() {
        use super::Simple;
        let mut buffer = String::new();
        let fmt = Simple::new();
        indent!(buffer, fmt, "{}\n{}", "Fantastic Mr Fox", "Is fantastic!");

        assert_eq!(
            buffer,
            "    Fantastic Mr Fox\n    Is fantastic!\n".to_string()
        );
    }

    #[test]
    fn it_renders_and_appends_to_a_buffer_with_indentation() {
        use super::Simple;
        use fmt::Render;
        struct SomeData;
        struct OtherData;
        struct MoreData;

        impl Render<SomeData> for Simple {
            fn render(&self, _data: &SomeData) -> Option<String> {
                let mut buf = String::from("One Line\nTwo Line\nRed Line\nBlue Line\n\n");
                nest!(buf, self, &OtherData);
                buf.push_str("--");
                Some(buf)
            }
        }

        impl Render<OtherData> for Simple {
            fn render(&self, _data: &OtherData) -> Option<String> {
                let mut buf = String::from("Deeper and deeper,\n");
                nest!(buf, self, &MoreData);
                Some(buf)
            }
        }

        impl Render<MoreData> for Simple {
            fn render(&self, _data: &MoreData) -> Option<String> {
                Some("The further I go".to_string())
            }
        }

        let mut buffer = String::new();
        let fmt = Simple::new();
        nest!(buffer, fmt, &SomeData);

        assert_eq!(
            buffer,
            "    One Line
    Two Line
    Red Line
    Blue Line

        Deeper and deeper,
            The further I go
    --
"
                .to_string()
        );
    }
}
