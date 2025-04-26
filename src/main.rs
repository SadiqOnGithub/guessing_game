use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please enter your guess: ");

    let secret_number = rand::random_range(1..=100);

    // println!("secret number {secret_number}");
    loop {
        let mut guess = String::new();
        println!("Please enter your guess");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Err(..) => {
                println!("that's not a number");
                continue;
            }
            Ok(num) => num,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("YOU WIN!!!");
                break;
            }
        }
    }
}
