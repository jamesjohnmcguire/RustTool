// extern crate rand;
// use std::{thread, time};

use ferris_says::say;
use std::io;
use std::io::{stdout, BufWriter};

fn main()
{
    println!("Hello, world!");

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}
