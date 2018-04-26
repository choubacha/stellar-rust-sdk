use std::io::{stdout, Stdout, Write};
use std::marker;

mod simple;
pub use self::simple::Simple;

/// The render trait can be used by the formatter to handle the calls to form
/// output strings. The formatter will call the preamble first when it's created,
/// then it will call the render when it's rendered, finally, when it's ended, it will
/// call the postamble.
pub trait Render<T> {
    fn preamble(&self) -> Option<String> {
        None
    }

    fn render(&self, _item: &T) -> Option<String> {
        None
    }

    fn postamble(&self) -> Option<String> {
        None
    }
}

/// A formatter that takes a write and a render and will output
/// lines of text based on what the render does.
pub struct Formatter<T, W, R>
where
    W: Write,
    R: Render<T>,
{
    writer: W,
    render: R,
    _marker: marker::PhantomData<T>,
}

impl<T, R> Formatter<T, Stdout, R>
where
    R: Render<T>,
{
    /// Starts the formatter with stdout.
    #[allow(dead_code)]
    pub fn start_stdout(render: R) -> Formatter<T, Stdout, R> {
        Formatter::<T, Stdout, R>::start(stdout(), render)
    }
}

impl<T, W, R> Formatter<T, W, R>
where
    W: Write,
    R: Render<T>,
{
    /// Starts the formatter with any struct that implements write.
    pub fn start(writer: W, render: R) -> Formatter<T, W, R>
    where
        W: Write,
        R: Render<T>,
    {
        Formatter {
            writer,
            render,
            _marker: marker::PhantomData,
        }._start()
    }

    fn _start(mut self) -> Self {
        if let Some(output) = self.render.preamble() {
            writeln!(&mut self.writer, "{}", output).expect("Failed writing to writer");
        }
        self
    }

    pub fn render(&mut self, item: &T) {
        if let Some(output) = self.render.render(item) {
            writeln!(&mut self.writer, "{}", output).expect("Failed writing to writer");
        }
    }

    pub fn stop(mut self) -> W {
        if let Some(output) = self.render.postamble() {
            writeln!(&mut self.writer, "{}", output).expect("Failed writing to writer");
        }
        self.writer
    }
}

#[cfg(test)]
mod render_tests {
    use super::*;

    use std::io::Cursor;

    struct Data {
        a: String,
        b: usize,
    }

    struct CSV;

    impl Render<Data> for CSV {
        fn preamble(&self) -> Option<String> {
            Some(String::from("a,b"))
        }

        fn render(&self, item: &Data) -> Option<String> {
            Some(format!("\"{}\",{}", item.a, item.b))
        }
    }

    #[test]
    fn renders_the_data() {
        let data = vec![
            Data {
                a: String::from("s1"),
                b: 1,
            },
            Data {
                a: String::from("s2"),
                b: 2,
            },
            Data {
                a: String::from("s3"),
                b: 3,
            },
        ];
        let cursor = Cursor::new(Vec::new());
        let mut render = Formatter::start(cursor, CSV);
        for datum in data {
            render.render(&datum)
        }
        let cursor = render.stop();
        let output = String::from_utf8(cursor.into_inner()).unwrap();
        assert_eq!(
            r#"a,b
"s1",1
"s2",2
"s3",3
"#,
            output
        );
    }
}
