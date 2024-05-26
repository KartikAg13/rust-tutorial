use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    println!("Guess the number!");

    //generate random number
    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    println!("The secret number is {}", secret_number);
    
    loop {
        println!("Please input your guess.");

        //declare a variable
        let mut guess = String::new();
    
        //get the input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        //convert string to integer using shadowing
        let guess:u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
        };
    
        println!("You guessed {}", guess);
    
        //compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
        }
    }
}
