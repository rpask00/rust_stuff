use std::io;
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    println!("please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("secret number is:{} ", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        println!("You guessed :{}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("match!");
                break;
            }
            Ordering::Greater => println!("too big!"),
            Ordering::Less => println!("too small!")
        }
    }
}
