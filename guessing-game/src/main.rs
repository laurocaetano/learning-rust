use rand::{thread_rng, Rng};

struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Guess {
        if value <= 100 {
            Guess { value }
        } else {
            panic!("The user guess must be a number between 1 and 100.");
        }
    }
}

fn main() {
    println!("Enter a guess from 1 to 100:");

    let mut user_guess = String::new();

    std::io::stdin()
        .read_line(&mut user_guess)
        .expect("Please enter a valid String");

    let guess = match user_guess.trim().parse() {
        Ok(v) => Guess::new(v),
        Err(_) => panic!("Please enter a number between 1 and 100."),
    };

    let random_number: u32 = thread_rng().gen_range(1..=100);

    if guess.value == random_number {
        println!("You've guessed correctly!");
    } else {
        println!(
            "You did not guess correctly. The random number was: {}.",
            random_number
        );
    }
}
