use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secre_number = rand::thread_rng().gen_range(1..=100);

    //   println!("The secret number is : {secre_number}");

    loop {
        println!("Please input your number");

        let mut guess = String::new();
        // let mut guess = "";

        io::stdin() // return an instance of std::io::Stdin, a handle input for your terminal
            .read_line(&mut guess) // append user input not overwrite
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secre_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
