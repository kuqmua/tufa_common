```
//mongodb run command
db.run_command(doc! {"ping": 1}, None).await?;
let filter = doc! { "author": "George Orwell" };
let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
//mongodb run command

//mongodb filter
let filter = doc! { "author": "George Orwell" };
let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
//mongodb filter

//mongodv drop collection example
collection.drop(None).await?;
//mongodb drop collection example

//mongodb app_name
client_options.app_name = Some("Rust Demo".to_string());
//mongodb app_name


//treadpool example
 use std::sync::mpsc::channel;
 use std::time::Instant;
 use threadpool::ThreadPool;

 fn main() {
     let since_fetch = Instant::now();
     let n_workers = 4;
     let n_jobs = 8000;
     let pool = ThreadPool::new(n_workers);

     let (tx, rx) = channel();
     for _ in 0..n_jobs {
         let tx = tx.clone();
         pool.execute(move || {
             println!("aaa");
             tx.send(1)
                 .expect("channel will be there waiting for the pool");
         });
     }

     assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8000);
     println!(
         "in {} sec {} mill",
         since_fetch.elapsed().as_secs(),
         since_fetch.elapsed().as_millis()
     );
 }
//treadpool example

//logger example
use log::LevelFilter;
use simplelog::{Config, TermLogger, TerminalMode};
TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
//logger example

//async tokio chat server example
https://youtu.be/4DqP57BHaXI
//async tokio chat server example

///////////////////////////////////////////////////////////////
// errors6.rs

// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Make these tests pass! Execute `rustlings hint errors6` for hints :)

// I AM NOT DONE

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    fn from_parse_int(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
    // TODO: add another error conversion function here.
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
```
