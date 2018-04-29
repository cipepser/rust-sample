extern crate getopts;
extern crate rustc_serialize;
extern crate csv;

use getopts::Options;
use std::env;
use std::fs;
use std::path::Path;
use std::error::Error;
use std::io;
use std::fmt;

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Csv(csv::Error),
    NotFound,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => err.fmt(f),
            CliError::Csv(ref err) => err.fmt(f),
            CliError::NotFound => write!(f, "No matching cities with a population were found."),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Csv(ref err) => err.description(),
            CliError::NotFound => "not found",
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> CliError {
        CliError::Csv(err)
    }
}

macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(::std::convert::From::from(err)),
    });
}

#[derive(Debug, RustcDecodable)]
struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    
    // 以下は存在しない可能性があるフィールド
    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

struct PopurationCount {
    city: String,
    country: String,
    count: u64,
}

fn print_usage(program: &str, opts: Options) {
    // println!("{}", opts.usage(&format!("Usage: {} [options] <data-path> <city>", program)));
    println!("{}", opts.usage(&format!("Usage: {} [options] <city>", program)));
}

// fn search<P: AsRef<Path>>(file_path: P, city: &str) -> Vec<PopurationCount> {
// fn search<P: AsRef<Path>>(file_path: P, city: &str)
            // -> Result<Vec<PopurationCount>, Box<Error+Send+Sync>> {
// fn search<P: AsRef<Path>>(file_path: &Option<P>, city: &str)
            // -> Result<Vec<PopurationCount>, Box<Error+Send+Sync>> {
fn search<P: AsRef<Path>>(file_path: &Option<P>, city: &str)
            -> Result<Vec<PopurationCount>, CliError> {
    let mut found = vec![];
    let input: Box<io::Read> = match *file_path {
        None => Box::new(io::stdin()),
        Some(ref file_path) => Box::new(try!(fs::File::open(file_path))),
    };
    // let file = fs::File::open(file_path).unwrap();
    // let mut rdr = csv::Reader::from_reader(file);
    let mut rdr = csv::Reader::from_reader(input);
    for row in rdr.decode::<Row>() {
        let row = row.unwrap();
        match row.population {
            None => { }
            Some(count) => if row.city == city {
                found.push(PopurationCount {
                    city: row.city,
                    country: row.country,
                    count: count,
                });
            },
        }
    }
    
    if found.is_empty() {
        // Err(From::from("No matching cities with a population were found."))
        Err(CliError::NotFound)
    } else {
        Ok(found)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    let mut opts = Options::new();
    opts.optopt("f", "file", "Choose an input file, instead of using STDIN.", "NAME");
    opts.optflag("h", "help", "Show this usage message.");
    // opts.optflag("q", "quiet", "Silences errors and warnings.");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };
    
    // opts.parse(&args[1..])
    // let matches = try!();
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let file = matches.opt_str("f");
    // let data_file = args[1].clone();
    let data_file = file.as_ref().map(Path::new);
    // let data_path = Path::new(&data_file);
    // let city = args[2].clone();
    let city = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    
    // let file = fs::File::open(data_path).unwrap();
    // let mut rdr = csv::Reader::from_reader(file);
    //
    // for row in rdr.decode::<Row>() {
    //     let row = row.unwrap();
    //     if row.city == city {
    //         println!("{}, {}: {:?}",
    //         row.city, row.country,
    //         row.population.expect("population count"));
    //     }
    // }
    
    // for pop in search(&data_path, &city) {
    //     println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
    // }
    
    match search(&data_file, &city) {
        Ok(pops) => {
            for pop in pops {
                println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
            }
        }
        Err(err) => println!("{}", err)
    }
    // let pops = search(&data_file, &city);
    // for pop in pops {
    //     println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
    // }
}

// 引数の順番で見ているので`cargo run`だとダメ。以下でちゃんといける。
// ❯ cargo build --release
// ❯ ./target/debug/city-pop -f uscitiespop.csv madison