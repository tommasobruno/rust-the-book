use std::{cmp::Ordering, io};

use rand::Rng;

fn test_slice(test: &str) -> &str {
    return &test[..2];
}

fn main() {
    println!("Guess a number");

    let random = rand::thread_rng().gen_range(1..100);

    let mut input = String::new();

    let res = test_slice("Test slice");
    println!("Result is {res}");

    loop {
        println!("Input your number");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read message");

        println!("Secret number is {random}");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input.cmp(&random) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("Your guess is correct!");
                break;
            }
            Ordering::Greater => println!("Too high!"),
        }
    }
}
