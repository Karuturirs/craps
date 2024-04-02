use rand::Rng;
use log::{info, error};

pub struct CrapsGame {
    rng: rand::rngs::ThreadRng,
    point: Option<u8>,
    amount : u16
}

impl CrapsGame {
    pub fn new() -> Self {
        CrapsGame {
            rng: rand::thread_rng(),
            point: None,
            amount : 2000
        }
    }

    pub fn roll_dice(&mut self) -> (u8, u8) {
        let die1: u8 = self.rng.gen_range(1..=6);
        let die2: u8 = self.rng.gen_range(1..=6);
        (die1, die2)
    }

    pub fn play(&mut self) {
        println!("Welcome to Rust Craps!");
        // Game loop
        loop {
            let (die1, die2) = self.roll_dice();
            let roll_total = die1 + die2;

            println!("You rolled: {} + {} = {}", die1, die2, roll_total);

            match self.point {
                None => {
                    match roll_total {
                        7 | 11 => {
                            println!("You win!");
                            break;
                        }
                        2 | 3 | 12 => {
                            println!("You lose!");
                            break;
                        }
                        _ => {
                            println!("Point is set to {}", roll_total);
                            self.point = Some(roll_total);
                        }
                    }
                }
                Some(point) => {
                    if roll_total == point {
                        println!("You rolled the point {}. You win!", point);
                        break;
                    } else if roll_total == 7 {
                        println!("You rolled a 7. You lose!");
                        break;
                    }
                }
            }
        }
    }
}
