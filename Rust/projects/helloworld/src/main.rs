use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Hello, world!");
    println!("Guess the number");
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => { println!("equal"); break; }
        }
    }
}

/* Shadowing - 
 * 1. To shadow a variable make sure, that the variable has served its purpose, if any, before the
 *    shadow variable is declared otherwise the compiler will shadow the variable and use the
 *    latest type where ever the new variable is used */
