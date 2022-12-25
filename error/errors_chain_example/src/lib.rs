//! Error chain is outdated, use 'thiserror' instead

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain::error_chain! {

        // Define additional `ErrorKind` variants.  Define custom responses with the
        // `description` and `display` calls.
        errors {
            FooErr(msg: &'static str) {
                description("FOO ERROR description")
                display("Foo msg is: '{}'", msg)
            }

            // You can also add commas after description/display.
            // This may work better with some editor auto-indentation modes:
            BarErr(msg:  &'static str) {
                description("BAR ERROR description"), // note the ,
                display("Bar msg is: '{}'", msg), // trailing comma is allowed
            }
        }

    }
}

mod example {

    use super::errors::*;
    // use super::errors::{Result, ErrorKind};
    use std::fs::File;

    /// Returns chained error with output like this:
    /// ```
    /// Error: unable to invoke 'foo()'
    /// Caused by: Foo msg is: 'bar was fail'
    /// Caused by: Bar msg is: 'unable to open file'
    /// Caused by: No such file or directory (os error 2)
    /// ```
    pub fn run() -> Result<()> {
        foo().chain_err(|| "unable to invoke 'foo()'")?;
        Ok(())
    }

    fn foo() -> Result<String> {
        bar("file name")
            .map(|_| "str".to_owned())
            .chain_err(|| ErrorKind::FooErr("bar was fail"))
    }

    fn bar(file: &str) -> Result<File> {
        // This operation will fail
        File::open(file).chain_err(|| ErrorKind::BarErr("unable to open file"))
    }
}

pub use self::example::run;
