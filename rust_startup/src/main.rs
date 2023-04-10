use std::io;
use rand::Rng; // 0.8.5

fn main() {
    
    println!("Guess the number (1 to 10):");

    loop {
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        if guess.contains("q") {
            break;
        }
        let num: i32 = rand::thread_rng().gen_range(0..10);

        if guess.contains(&num.to_string()) {
            println!("You guessed correctly!");
            break;
        } else {
            
            println!("You guessed: {} and the number was {}", guess, num);
            println!("Try again!");
        }

    };

    println!("Thank you for playing! Bye!");
}
