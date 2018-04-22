// extern crate phrases;
//
// use phrases::english::{greetings, farewells};
// use phrases::japanese;
//
// fn main() {
//     println!("Hello in English: {}", greetings::hello());
//     println!("Goodby in English: {}", farewells::goodbye());
//
//     println!("Hello in Japanese: {}", japanese::hello());
//     println!("Goodby in Japanese: {}", japanese::goodbye());
// }

extern crate phrases as sayings;

use sayings::japanese::greetings as ja_greetings;
use sayings::japanese::farewells::*;
use sayings::english::{self, greetings as en_greetings, farewells as en_farewells};

fn main() {
    println!("Hello in English: {}", en_greetings::hello());
    println!("And in Japanese: {}", ja_greetings::hello());
    println!("Goodbye in English: {}", english::farewells::goodbye());
    println!("Again: {}", en_farewells::goodbye());
    println!("And in Japanese: {}", goodbye());
}