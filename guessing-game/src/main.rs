use rand::{thread_rng, Rng};

fn main() {
    println!("Enter a guess from 1 to 100:");

    let mut user_guess = String::new();

    std::io::stdin()
        .read_line(&mut user_guess)
        .expect("Please enter a valid String");

    let user_guess: u32 = match user_guess.trim().parse() {
        Ok(v) => v,
        Err(_) => panic!("Expected a value between 1 and 100. Please enter a valid number."),
    };

    let random_guess: u32 = thread_rng().gen_range(1..=100);

    if user_guess == random_guess {
        println!("You've guessed correctly!");
    } else {
        println!(
            "You did not guess correctly. The random number was: {}.",
            random_guess
        );
    }
}
