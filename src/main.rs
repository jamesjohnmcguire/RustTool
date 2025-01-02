// extern crate rand;
// use std::{thread, time};

use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering;
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

    let mut generator = rand::thread_rng();

    let secret_number = generator.gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new();

    let instance = io::stdin();

    let result = instance.read_line(&mut guess);

    result.expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
