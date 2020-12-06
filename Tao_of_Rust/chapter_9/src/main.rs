#![feature(try_trait)]
fn main() {
  {
    options_learning();
  }
  {
    result_learning();
  }
}

fn options_learning() {
  fn get_shortest(names: Vec<&str>) -> Option<&str> {
    if names.len() > 0 {
      let mut shortest = names[0];
      for name in names.iter() {
        if name.len() < shortest.len() {
          shortest = *name;
        }
      }
      Some(shortest)
    } else {
      None
    }
  }
  {
    fn show_shortest(names: Vec<&str>) -> &str {
      match get_shortest(names) {
        Some(shortest) => shortest,
        None => "Not Found",
      }
    }

    assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
    assert_eq!(show_shortest(Vec::new()), "Not Found");
  }
  {
    fn show_shortest(names: Vec<&str>) -> &str {
      get_shortest(names).unwrap_or("Not Found")
      // get_shortest(names).unwrap_or_else( || "Not Found")
      // this will panicked and out put log
      // get_shortest(names).expect("Not Found")
    }

    assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
    assert_eq!(show_shortest(Vec::new()), "Not Found");
  }
  {
    fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
      match get_shortest(names) {
        Some(shortest) => Some(shortest.len()),
        None => None,
      }
    }
    assert_eq!(get_shortest_length(vec!["Uku", "Felipe"]), Some(3));
    assert_eq!(get_shortest_length(Vec::new()), None);
  }
  {
    fn get_shortest_length(vec: Vec<&str>) -> Option<usize> {
      get_shortest(vec).map(|name| name.len())
    }
    assert_eq!(get_shortest_length(vec!["Uku", "Felipe"]), Some(3));
    assert_eq!(get_shortest_length(Vec::new()), None);
  }
  {
    fn double(value: f64) -> f64 {
      value * 2.0
    }
    fn square(value: f64) -> f64 {
      value.powi(2 as i32)
    }
    fn inverse(value: f64) -> f64 {
      value * -1.
    }
    fn log(value: f64) -> Option<f64> {
      match value.log2() {
        x if x.is_normal() => Some(x),
        _ => None,
      }
    }
    fn sqrt(value: f64) -> Option<f64> {
      match value.sqrt() {
        x if x.is_normal() => Some(x),
        _ => None,
      }
    }

    let number: f64 = 20.0;
    let result = Option::from(number)
      .map(inverse)
      .map(double)
      .map(inverse)
      .and_then(log)
      .map(square)
      .and_then(sqrt);
    match result {
      Some(x) => assert_eq!(x, 5.321928094887363),
      None => panic!("error"),
    }
  }
}

fn result_learning() {
  {
    let n = "1";
    assert_eq!(n.parse::<i32>(), Ok(1));
  }
  {
    use std::num::ParseIntError;
    type ParseResult = Result<i32, ParseIntError>;
    fn square(number_str: &str) -> ParseResult {
      number_str.parse::<i32>().map(|n| n.pow(2))
    }

    let i = "1";
    assert_eq!(Ok(1), square(&i));
  }
  {
    #[allow(dead_code)]
    fn sum_text() {
      use std::env;
      use std::fs::File;
      use std::io::prelude::*;
      let args: Vec<String> = env::args().collect();
      println!("{:?}", args);
      let filename = &args[1];
      let mut f = File::open(filename).unwrap();
      let mut contents = String::new();
      f.read_to_string(&mut contents).unwrap();
      let mut sum = 0;
      for c in contents.lines() {
        let n: i32 = c.parse().unwrap();
        sum += n;
      }
      println!("{:?}", sum);
    }
  }
  {
    use std::env;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::process;
    #[allow(non_camel_case_types)]
    type ParseResult<i32> = Result<i32, Box<dyn Error>>;
    #[allow(dead_code)]
    fn sum_text() {
      let args: Vec<String> = env::args().collect();
      let filename = &args[1];
      println!("In file {}", filename);
      match run(filename) {
        Ok(n) => println!("{:?}", n),
        Err(e) => {
          println!("main error {}", e);
          process::exit(1);
        }
      }
    }

    #[allow(dead_code)]
    #[allow(deprecated)]
    fn run(filename: &str) -> ParseResult<i32> {
      File::open(filename)
        .map_err(|e| e.into())
        .and_then(|mut f| {
          let mut contents = String::new();
          f.read_to_string(&mut contents)
            .map_err(|e| e.into())
            .map(|_| contents)
        })
        .and_then(|contents| {
          let mut sum = 0;
          for c in contents.lines() {
            match c.parse::<i32>() {
              Ok(n) => {
                sum += n;
              }
              Err(err) => {
                let err: Box<dyn Error> = err.into();
                println!("error info:{} ,cause: {:?}", err.description(), err.cause());
              } // Err(err) => {return Err(From::from(err))},
            }
          }
          Ok(sum)
        })
    }
  }
  #[allow(dead_code)]
  {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::{fmt, io, num};
    #[allow(non_camel_case_types)]
    type ParseResult<i32> = std::result::Result<i32, CliError>;
    #[derive(Debug)]
    enum CliError {
      Io(io::Error),
      Parse(num::ParseIntError),
    }

    impl fmt::Display for CliError {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
          CliError::Io(ref err) => write!(f, "IO error: {}", err),
          CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
      }
    }
    #[allow(deprecated)]
    impl Error for CliError {
      fn description(&self) -> &str {
        match *self {
          CliError::Io(ref err) => err.description(),
          CliError::Parse(ref err) => Error::description(err),
        }
      }
      fn cause(&self) -> Option<&dyn Error> {
        match *self {
          CliError::Io(ref err) => Some(err),
          CliError::Parse(ref err) => Some(err),
        }
      }
    }
    #[allow(deprecated)]
    fn run(filename: &str) -> ParseResult<i32> {
      File::open(filename)
        .map_err(CliError::Io)
        .and_then(|mut f| {
          let mut contents = String::new();
          f.read_to_string(&mut contents)
            .map_err(CliError::Io)
            .map(|_| contents)
        })
        .and_then(|contents| {
          let mut sum = 0;
          for c in contents.lines() {
            match c.parse::<i32>() {
              Ok(n) => {
                sum += n;
              }
              Err(err) => {
                let err = CliError::Parse(err);
                println!(
                  "Error info: {} \n Cause by {:?}",
                  err.description(),
                  err.cause()
                );
              } // Err(err) => {return Err(CliError::Parse(err));}
            }
          }
          Ok(sum)
        })
    }
  }
  #[allow(dead_code)]
  {
    use std::option::NoneError;
    use std::{error::Error, fmt, io, num};
    #[allow(non_camel_case_types)]
    type ParseResult<i32> = std::result::Result<i32, CliError>;
    #[derive(Debug)]
    enum CliError {
      Io(io::Error),
      Parse(num::ParseIntError),
      NoneError(NoneError),
    }

    impl fmt::Display for CliError {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
          CliError::Io(ref err) => write!(f, "IO error: {}", err),
          CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
          CliError::NoneError(ref err) => write!(f, "Command args error: {:?}", err),
        }
      }
    }

    #[allow(deprecated)]
    impl Error for CliError {
      fn description(&self) -> &str {
        match *self {
          CliError::Io(ref err) => err.description(),
          CliError::Parse(ref err) => Error::description(err),
          CliError::NoneError(ref _err) => "NoneError",
        }
      }
      fn cause(&self) -> Option<&dyn Error> {
        match *self {
          CliError::Io(ref err) => Some(err),
          CliError::Parse(ref err) => Some(err),
          _ => None,
        }
      }
    }

    impl From<io::Error> for CliError {
      fn from(err: io::Error) -> CliError {
        CliError::Io(err)
      }
    }

    impl From<num::ParseIntError> for CliError {
      fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
      }
    }

    impl From<NoneError> for CliError {
      fn from(err: NoneError) -> CliError {
        CliError::NoneError(err)
      }
    }
    use std::fs::File;
    use std::io::Read;
    fn run(filename: Option<String>) -> ParseResult<i32> {
      let mut file = File::open(filename?)?;
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
      let mut sum = 0;
      for c in contents.lines() {
        let n: i32 = c.parse::<i32>()?;
        sum += n;
      }
      Ok(sum)
    }

    use std::env;
    use std::process;
    fn sum_text() {
      let filename = env::args().nth(1);
      match run(filename) {
        Ok(n) => {
          println!("{:?}", n)
        }
        Err(e) => {
          println!("main error {:?}", e);
          process::exit(1);
        }
      }
    }
  }
  {}
}
