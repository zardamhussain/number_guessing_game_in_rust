use rand::Rng;
use std::{io::{self, Write}, cmp::Ordering::{Less, Greater, Equal}};
use colored::*;

fn main() {

    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(1..101);
    
    println!("{}", "\t╔═════════════════════════════════════════════════════╗".blue().bold());
    println!("{}", "\t║            🎯 Welcome to the Number Guess! 🎯       ║".blue().bold());
    println!("{}", "\t╚═════════════════════════════════════════════════════╝".blue().bold());
    println!("\n{}", "🚀 Select a number between 1-100 🚀".bright_blue().bold().italic());
    
    loop {
        
        let mut input = String::new();
        print!("{}", "👉 Enter your guess: ".yellow().bold());

        if io::stdout().flush().is_err() {
            println!("{}", "⚠️  Failed to flush stdout.".red().bold());
            continue;
        }
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                println!("{}", "⚠️  Failed to read input.".red().bold());
                continue;
            }
        }
        
        let parsed_input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⛔ Please enter a valid integer.".red().bold().underline());
                continue;
            }
        };
        
        match parsed_input.cmp(&random_number) {
            Less => println!("{}", "⬇️  Too low! Give it another go!".cyan().bold().italic()),
            Greater => println!("{}", "⬆️  Too high! Try something smaller!".magenta().bold().italic()),
            Equal => {
                println!("\n{}", format!("🎉 Congratulations! You guessed the right number: {}", random_number).green().bold().underline());
                println!("{}", "🌟 Amazing! You're a Number Guessing Master! 🌟".green().bold());
                break;
            }
        }
    }

    println!("\n{}", "👋 Thanks for playing! See you next time!".bright_white().bold().italic());
    println!("{}", "══════════════════════════════════════════".blue().bold());

}
