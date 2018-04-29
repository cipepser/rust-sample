// Option と Result を合成する
// use std::env;
//
// fn double_arg(mut argv: env::Args) -> Result<i32, String> {
//     argv.nth(1)
//         .ok_or("Please give at least one argument".to_owned())
//         .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
//         .map(|n| 2 * n)
// }
//
// fn main() {
//     match double_arg(env::args()) {
//         Ok(n) => println!("{}", n),
//         Err(err) => println!("Error: {}", err),
//     }
// }

// ❯ ./target/debug/error
// Error: Please give at least one argument
//
// ❯ ./target/debug/error a
// Error: invalid digit found in string
//
// ❯ ./target/debug/error 1
// 2

// コンビネータの限界
// use std::fs::File;
// use std::io::Read;
// use std::path::Path;
//
// fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
//     File::open(file_path)
//         .map_err(|err| err.to_string())
//         .and_then(|mut file| {
//             let mut contents = String::new();
//             file.read_to_string(&mut contents)
//                 .map_err(|err| err.to_string())
//                 .map(|_| contents)
//         })
//         .and_then(|contents| {
//             contents.trim().parse::<i32>()
//                 .map_err(|err| err.to_string())
//         })
//         .map(|n| 2 * n)
// }
//
// fn main() {
//     match file_double("foobar") {
//         Ok(n) => println!("{}", n),
//         Err(err) => println!("Error: {}", err),
//     }
// }

// 早期リターン
// use std::fs::File;
// use std::io::Read;
// use std::path::Path;
//
// fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
//     let mut file = match File::open(file_path) {
//         Ok(file) => file,
//         Err(err) => return Err(err.to_string()),
//     };
//     let mut contents = String::new();
//     if let Err(err) = file.read_to_string(&mut contents) {
//         return Err(err.to_string());
//     }
//     let n: i32 = match contents.trim().parse() {
//         Ok(n) => n,
//         Err(err) => return Err(err.to_string()),
//     };
//     Ok(2 * n)
// }
//
// fn main() {
//     match file_double("foobar") {
//         Ok(n) => println!("{}", n),
//         Err(err) => println!("Error: {}", err),
//     }
// }

// try! マクロ
// macro_rules! try {
//     ($e:expr) => (match $e {
//         Ok(val) => val,
//         Err(err) => return Err(err),
//     });
// }
//
// use std::fs::File;
// use std::io::Read;
// use std::path::Path;
//
// fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
//     let mut file = try!(File::open(file_path).map_err(|e| e.to_string()));
//     let mut contents = String::new();
//     try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
//     let n = try!(contents.trim().parse::<i32>().map_err(|e| e.to_string()));
//     Ok(2 * n)
// }
//
// fn main() {
//     match file_double("foobar") {
//         Ok(n) => println!("{}", n),
//         Err(err) => println!("Error: {}", err),
//     }
// }

// 独自のエラー型を定義する
// use std::io;
// use std::num;
//
// #[derive(Debug)]
// enum CliError {
//     Io(io::Error),
//     Parse(num::ParseIntError)
// }
//
// macro_rules! try {
//     ($e:expr) => (match $e {
//         Ok(val) => val,
//         Err(err) => return Err(err),
//     });
// }
//
// use std::fs::File;
// use std::io::Read;
// use std::path::Path;
//
// fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
//     let mut file = try!(File::open(file_path).map_err(CliError::Io));
//     let mut contents = String::new();
//     try!(file.read_to_string(&mut contents).map_err(CliError::Io));
//     let n = try!(contents.trim().parse::<i32>().map_err(CliError::Parse));
//     Ok(2 * n)
// }
//
// fn main() {
//     match file_double("foobar") {
//         Ok(n) => println!("{}", n),
//         Err(err) => println!("Error: {:?}", err),
//     }
// }

// Error トレイト
// use std::io;
// use std::num;
//
// #[derive(Debug)]
// enum CliError {
//     Io(io::Error),
//     Parse(num::ParseIntError)
// }
//
// use std::error;
// use std::fmt;
//
// impl fmt::Display for CliError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             CliError::Io(ref err) => write!(f, "IO error: {}", err),
//             CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
//         }
//     }
// }
//
// impl error::Error for CliError {
//     fn description(&self) -> &str {
//         match *self {
//             CliError::Io(ref err) => err.description(),
//             CliError::Parse(ref err) => err.description(),
//         }
//     }
//
//     fn cause(&self) -> Option<&error::Error> {
//         match *self {
//             CliError::Io(ref err) => Some(err),
//             CliError::Parse(ref err) => Some(err),
//         }
//     }
// }

// 本当のtry!マクロ
// macro_rules! try {
//     ($e:expr) => (match $e {
//         Ok(val) => val,
//         Err(err) => return Err(::std::convert::From::from(err)),
//     });
// }
//
// use std::error::Error;
// use std::fs::File;
// use std::io::Read;
// use std::path::Path;
//
// fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<Error>> {
//     let mut file = try!(File::open(file_path));
//     let mut contents = String::new();
//     try!(file.read_to_string(&mut contents));
//     let n = try!(contents.trim().parse::<i32>());
//     Ok(2 * n)
// }
//
// fn main() {
//     match file_double("foobar") {
//         Ok(n) => println!("{}", n),
//         Err(err) => println!("Error: {:?}", err),
//     }
// }

// 独自のエラー型を合成する
use std::fs::File;
use std::io::{self, Read};
use std::num;
use std::path::Path;

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError),
}

// fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
//     let mut file = try!(File::open(file_path).map_err(CliError::Io));
//     let mut contents = String::new();
//     try!(file.read_to_string(&mut contents).map_err(CliError::Io));
//     let n: i32 = try!(contents.trim().parse().map_err(CliError::Parse));
//     Ok(2 * n)
// }

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::ParseInt(err)
    }
}

impl From<num::ParseFloatError> for CliError {
    fn from(err: num::ParseFloatError) -> CliError {
        CliError::ParseFloat(err)
    }
}

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let n: i32 = try!(contents.trim().parse());
    Ok(2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}








