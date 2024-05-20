use std::cmp::Ordering;
use crate::utility::random;

const MENU_TEMPLATE: &str =
    "**********************\n\
     ****  Guess Game  ****\n\
     * A number (1-100)   *\n\
     * randomly generated *\n\
     **********************";

fn game_statistics(guess: &Guess, egg: &u8) {
    println!("**********************\n\
             ****  Statistics  ****\n\
              Guessed: {} times\n\
              Target:  {}\n\
             **********************",
             guess.guessed_times,
             egg);
}

#[derive(Debug, PartialEq)]
enum GuessError {
    InvalidInput(String),
    Parse,
    Quit,
    Restart,
}

struct Guess {
    maximum_guess_times: u8,
    guessed_times: u8,
    guessed: u8,
    is_win: bool,
}

impl Guess {
    fn read_input(&self) -> Result<String, GuessError> {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess)
            .map_err(|e| GuessError::InvalidInput(e.to_string()))?;
        if guess.trim().is_empty() {
            return Err(GuessError::InvalidInput("empty input".to_string()));
        }
        Ok(guess.trim().to_string())
    }

    fn parse_input(&mut self) -> Result<(), GuessError> {
        let input = self.read_input()?;
        match input.parse::<u8>() {
            Ok(v) => Ok(self.guessed = v),
            Err(_) => {
                if input == "Q" {
                    return Err(GuessError::Quit);
                } else if self.is_win || self.over_limits() {
                    return
                        if input == "Y" { Err(GuessError::Restart) } else { Err(GuessError::Quit) };
                }
                Err(GuessError::Parse)
            }
        }
    }

    fn over_limits(&self) -> bool {
        self.guessed_times > self.maximum_guess_times
    }
    fn compare(&self, egg: &u8) -> Ordering {
        self.guessed.cmp(egg)
    }
}


fn init_guess() -> Guess {
    Guess {
        maximum_guess_times: 5,
        guessed_times: 0,
        guessed: 0,
        is_win: false,
    }
}

fn generate_egg() -> u8 {
    random::generated_by(1..=100)
}


pub fn run() {
    let mut egg: u8 = generate_egg();
    let mut is_restarting = false;
    let mut game = init_guess();

    println!("{MENU_TEMPLATE}");

    loop {
        if is_restarting {
            egg = generate_egg();
            game = init_guess();
            is_restarting = false;
        }

        if !game.is_win {
            game.guessed_times += 1;

            if game.over_limits() {
                println!("Maximum guess: {}, You guessed: {}",
                         game.maximum_guess_times,
                         game.guessed_times - 1);
                println!("Do you want to continue? (Y)");
            } else {
                println!("#{} Please input your guess:", game.guessed_times);
            }
        }

        match game.parse_input() {
            Ok(_) => {
                match game.compare(&egg) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("Bingo!");
                        game.is_win = true;
                        game_statistics(&game, &egg);
                        println!("Do you want to play again? (Y)");
                    }
                }
            }
            Err(e) => {
                match e {
                    GuessError::Quit => break,
                    GuessError::Restart => {
                        is_restarting = true;
                        continue;
                    }
                    _ => continue,
                }
            }
        }
    }
    println!("Game Over");
}