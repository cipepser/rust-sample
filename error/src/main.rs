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
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents.trim().parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| 2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}