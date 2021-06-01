pub mod pointing;

use std::fs::{self, File};
use std::io::{self, Write};
use std::process;

use u06::sudoku::{self, CollectedSudokuError};

fn main() {
    explore_errors_with_sudoku();
}

pub fn explore_errors_with_sudoku() {
    let mut grid = sudoku::Grid::new(9, 9);

    // The basic error type in Go is `error` which provides an `Error()`
    // method with the description of the error.
    //
    // This is used for cases where all we care is detecting that some
    // error happened and possibly getting a description of the error for
    // reporting or diagnostics. All other styles of errors can be used this
    // way.
    //
    // The Rust equivalent of this will be `dyn std::error::Error` trait.
    // It's common to use a crate like `anyhow` to make this style of errors.
    if let Err(err) = grid.set_with_basic_error(10, 0, 5) {
        eprintln!("{}", err);
    }

    // The next style of errors is using error values which are compared to variables
    // defined in a package. This allows comparison (by comparing pointers) by
    // client code to check the kind of errors. These error variables are prefixed with
    // "Err" by convention.
    //
    //  ```
    //  package foo
    //
    //  var ErrBaz = errors.New("bane failed")
    //  var ErrCaz = errors.New("cane failed")
    //
    //  func Bar() error {
    //     return ErrBaz  // note that the exact
    //  }
    //
    //
    //  // in client code
    //  _, err := foo.Bar()
    //  if err == foo.ErrBaz {
    //     // ...
    //  } // ...etc.
    //  ```
    //
    //
    // The Rust equivalent of this is to return an `enum` with possible error
    // kinds as variants. Since Rust enums can have values, this can also be
    // used for style of errors that capture additional info.
    if let Err(err) = grid.set_with_enumerated_error(10, 0, 5) {
        use u06::sudoku::EnumeratedSudokuError;
        let msg = match err {
            EnumeratedSudokuError::InvalidCellIndex => {
                format!("This is the wrong place, man: {}.", err)
            }
            EnumeratedSudokuError::InvalidDigit(digit) => {
                format!("{}, it's bad m'kay: {}.", digit, err)
            }
            err => err.to_string(),
        };
        eprintln!("{}", msg);
        // process::exit(1);
    }

    // Custom error types are used to capture additional error-specific
    // information in both Go and Rust.
    //
    // In Go functions always return the standard `error` interface and never
    // a concrete type or a pointer to a concrete error type.
    // Go uses type assertions to check and access the underlying error types.
    //
    // if ferr, ok := err.(FooError); ok {
    //    // access FooError specific data
    // }
    //
    // It is conventional to alwyas use an explicity `return nil` when
    // there are no errors to avoid the possibility of accidentally turning
    // a nil pointer to an interface with a null pointer which will not compare to `nil`.
    //

    // In Rust, it's more conventional to return concrete error types in libraries.
    // A crate like anyhow can be used to make using generic errors more
    // convenient:
    if let Err(err) = grid.set_with_collected_error(10, 0, 21) {
        match err.downcast_ref::<CollectedSudokuError>() {
            Some(err) => {
                eprintln!("Failed:");
                for err in err.errors() {
                    eprintln!(" 'cause {}", err);
                }
                eprintln!();
            }
            _ => eprintln!("Failed: {}", err),
        }
        // process::exit(1);
    }
}

pub fn explore_errors_by_writing_files() {
    fn write_bs() -> io::Result<()> {
        // Go uses `defer` to ensure cleanup code is run at the
        // end of a function:
        // f, err := os.Create("bs.txt")
        // if err != nil {
        //    return err;
        // }
        // defer f.Close()
        //
        // Rust uses C++ style RAAI (Resource Acquisition Is Initialization)
        // where the drop implementation associated with the resource is
        // automatically called when the var goes out of scope.
        //
        // Go's defer scheme allows arbitrary clean-up code, but the RAII style
        // clean-up is bound to the clean-up defined for that type. Go's defer is a more
        // general construct and can be used to run anything that we want to guarantee
        // to be run at the end of a function.
        let mut f = File::create("bs.txt")?;

        // this pattern of checking and returning error
        // (propagating errors) is common to both Go and Rust
        // _, err = fmt.Fprintln(f, "synergy")
        if let Err(err) = writeln!(f, "{}", "synergy") {
            return Err(err);
        }
        // the `?` operator is a shortcut for the above pattern
        writeln!(f, "{}", "leverage")?;
        writeln!(f, "{}", "cloud")?;
        Ok(())
    }

    fn total_bs() -> io::Result<()> {
        write_bs()?;
        Ok(())
    }

    if let Err(err) = total_bs() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

pub fn explore_errors_by_reading_files() {
    // Go and Rust follow the same model for error handling---
    // treating errors as values returned along with the
    // return value (as opposed to using a global error variable,
    // or exceptions)
    // Go uses it's multiple return value support to return the
    // error value (conventionally the last value of a multiple
    // return)
    //
    // files, err := ioutil.ReadDir(".")
    // if err != nil {
    //    // handle error
    // }
    //
    // Rust does not support multiple return values (but can be
    // emulated by returning a tuple), Rust returns errors as
    // the Err variant of a Result enum.
    //
    // One difference with Go is that in Go, the function can return
    // both the value and the error which can be used to return some
    // value even when there is an error (e.g. partial result up to the
    // point of failure) although in most cases, it's considered
    // unspecified and should not be used (it most probably has the value's
    // zero value). This is not possible in Rust because Result is either a
    // result value or and error value, not both. Any such values will need
    // to be part of the Error
    let files = match fs::read_dir(".") {
        Ok(files) => files,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    // Go's filesystem API makes some sensible assumptions like assuming/converting
    // path names to UTF-8 and ignoring errors. All those assumptions have to
    // be made explicit in the Rust filesystem API. This gives more file-grained
    // control over various corner cases, but it also makes the API more
    // cumbersome to use.
    for file in files {
        println!(
            "{}",
            file.unwrap()
                .path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );
    }
}
