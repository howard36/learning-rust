use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut rng = rand::thread_rng();
    let secret = rng.gen_range(1..101);
    println!("The secret number is {}", secret);

    println!("Guess a number");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed {}, len = {}", guess, guess.len());

        let guess = guess.trim().parse::<i32>();

        match guess {
            Ok(n) => {
                println!("Good guess: {}", n);
                match n.cmp(&secret) {
                    Ordering::Less => println!("Too small"),
                    Ordering::Greater => println!("Too big"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    },
                }
            },
            Err(_) => println!("Bad guess, try again"),
        }
    }
        

}
