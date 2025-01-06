use std::io;
use std::cmp::Ordering;  //scope of enum when 2 things are being compared
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess!");

        let mut guess: String = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
    
        let guess: u32 = match guess.trim().parse() { //here shadowing is implemented on a variable guess by converting type and preserving same value.
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small!".red()),
            Ordering::Greater => println!("{}","Too Large!".red()),
            Ordering::Equal => {
                println!("{}","You Win!".green());
                break;
        },
    }

}
}
