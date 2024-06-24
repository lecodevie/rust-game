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
              Maximum: {} times\n\
              Guessed: {} times\n\
              Target:  {}\n\
             **********************",
             guess.maximum_guess_times,
             guess.guessed_times,
             egg);
}

#[derive(Debug)]
enum GuessError {
    Invalid(String),
}

#[derive(Debug)]
enum GuessState {
    Input,
    Quit,
    Restart,
    Win,
    Confirm,
}

enum Token {
    Value(u8),
    Code(char),
}

trait Compare {
    fn compare(&self, other: &u8) -> Ordering;
}

impl Compare for u8 {
    fn compare(&self, other: &u8) -> Ordering {
        match self {
            x if x < other => Ordering::Less,
            x if x > other => Ordering::Greater,
            _ => Ordering::Equal
        }
    }
}

struct Guess {
    maximum_guess_times: u8,
    guessed_times: u8,
    is_win: bool,
}

impl Guess {
    fn read_input(&self) -> Option<String> {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Can't read line!");
        let input = guess.trim();
        if input.is_empty() {
            None
        } else {
            Some(input.into())
        }
    }

    fn parse_input(&self) -> Result<Token, GuessError> {
        if let Some(input) = self.read_input() {
            if let Ok(value) = input.parse::<u8>() {
                Ok(Token::Value(value))
            } else {
                match input {
                    confirm if confirm == "Y" => Ok(Token::Code('Y'))
                    ,
                    quit if quit == "Q" => Ok(Token::Code('Q')),
                    _ => Err(GuessError::Invalid("CODE".to_string()))
                }
            }
        } else {
            Err(GuessError::Invalid("EMPTY".to_string()))
        }
    }

    fn parse_token(&mut self, egg: &u8, input: bool) -> Result<GuessState, GuessError> {
        if input {
            return Ok(GuessState::Input);
        }
        match self.parse_input() {
            Ok(token) => {
                let state = match token {
                    Token::Value(ref value) => {
                        if self.over_limits() {
                            GuessState::Confirm
                        } else if self.wining(value, egg) {
                            GuessState::Win
                        } else {
                            GuessState::Input
                        }
                    }
                    Token::Code(c) => match c {
                        c if c == 'Y' && (self.is_win || self.over_limits())
                        => GuessState::Restart,
                        c if c == 'Q'
                        => GuessState::Quit,
                        _ => return Err(GuessError::Invalid("UNKNOWN".to_string()))
                    },
                };
                Ok(state)
            }
            Err(e) => if self.over_limits() { Ok(GuessState::Confirm) } else { Err(e) }
        }
    }

    fn increment_times(&mut self) {
        if !self.over_limits() {
            self.guessed_times += 1;
        } else {
            self.guessed_times = self.maximum_guess_times;
        }
    }
    fn over_limits(&self) -> bool {
        self.guessed_times >= self.maximum_guess_times
    }

    fn wining(&mut self, token: &u8, egg: &u8) -> bool {
        self.is_win = match token.compare(egg) {
            Ordering::Less => {
                println!("Too small!");
                false
            }
            Ordering::Greater => {
                println!("Too big!");
                false
            }
            Ordering::Equal => {
                println!("Bingo!");
                true
            }
        };
        self.is_win
    }
}

fn init_guess() -> Guess {
    Guess {
        maximum_guess_times: 5,
        guessed_times: 0,
        is_win: false,
    }
}

fn generate_egg() -> u8 {
    random::generated_by(1..=100)
}

pub fn run() {
    let mut is_restarting = false;
    let mut is_starting = true;

    let mut egg = generate_egg();
    let mut game = init_guess();

    println!("{MENU_TEMPLATE}");

    loop {
        if is_restarting {
            egg = generate_egg();
            game = init_guess();
            is_restarting = false;
            println!("{egg}");
        }

        game.increment_times();

        if let Ok(ref state) = game.parse_token(&egg, is_starting) {
            match state {
                GuessState::Input => {
                    println!("#{} Please enter your guess:", game.guessed_times);
                }
                GuessState::Win => {
                    println!("Do you want to play again? (Y/Q)");
                }
                GuessState::Confirm => {
                    println!("Maximum guess: {}, You guessed: {}",
                             game.maximum_guess_times,
                             game.guessed_times);
                    println!("Do you want to continue? (Y/Q)");
                }
                GuessState::Restart => {
                    is_restarting = true;
                    is_starting = true;
                }
                GuessState::Quit => break,
            }
        } else {
            println!(" >>> Invalid Input <<< ");
        }

        if !(is_starting && is_restarting) {
            is_starting = false;
        }
    }
    game_statistics(&game, &egg);
    println!("Game Over");
}