extern crate rand;

use rand::Rng;

fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");
    let mut guess: String = String::new();
    guess = "test".parse().unwrap();
    //    io::stdin()
    //        .read_line(&mut guess)
    //        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
