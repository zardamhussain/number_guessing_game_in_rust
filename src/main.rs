use rand::Rng;
use std::{io::{self, Write}, cmp::Ordering::{Less, Greater, Equal}};
use colored::*;

fn main() {

    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(1..101);
    
    println!("{}", "\t╔═════════════════════════════════════════════════════╗".blue().bold());
    println!("{}", "\t║            🎯 Welcome to the Number Guess! 🎯       ║".blue().bold());
    println!("{}", "\t╚═════════════════════════════════════════════════════╝".blue().bold());
    
    println!("\n{}", "🔧 Please select a difficulty level: ".bright_yellow().bold());
    println!("{}", "1) 🟢 Easy (10 chances)".green().bold());
    println!("{}", "2) 🟡 Medium (5 chances)".yellow().bold());
    println!("{}", "3) 🔴 Hard (3 chance)".red().bold());

    let difficulty = loop {
        let mut input = String::new();
        print!("{}", "👉 Enter 1 for Easy, 2 for Medium, or 3 for Hard: ".cyan().bold());

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
        
        match input.trim() {
            "1" => break "easy",
            "2" => break "medium",
            "3" => break "hard",
            _ => {
                println!("{}", "⛔ Invalid selection. Please select 1, 2, or 3.".red().bold().underline());
                continue;
            }
        }
    };

    let chances = match difficulty {
        "easy" => 10,
        "medium" => 5,
        "hard" => 3,
        _ => 1
    };

    println!("\n{}", format!("🌟 You selected '{}' mode! You have {} chance(s).", difficulty, chances).bright_cyan().bold().italic());
    
    let mut remaining_chances = chances;
    
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
        
        remaining_chances -= 1;
        
        match parsed_input.cmp(&random_number) {
            Less if remaining_chances > 0 => println!("{}", format!("⬇️  Too low! Try again, {} chance(s) remaining!", remaining_chances).cyan().bold().italic()),
            Greater if remaining_chances > 0 => println!("{}", format!("⬆️  Too high! Try again, {} chance(s) remaining!", remaining_chances).magenta().bold().italic()),
            Equal => {
                println!("\n{}", format!("🎉 Congratulations! You guessed the right number: {}", random_number).green().bold().underline());
                println!("{}", "🌟 Amazing! You're a Number Guessing Master! 🌟".green().bold());
                break;
            },
            _ => {}
        }

        if remaining_chances == 0 {
            println!("\n{}", "💀 You ran out of chances! Better luck next time!".red().bold());
            println!("{}", "🤡 Haha, you really thought you'd win? Too bad!".red().bold().italic());
            break;
        }

    }

    println!("\n{}", "👋 Thanks for playing! See you next time!".white().italic().dimmed());
    println!("{}", "══════════════════════════════════════════".blue().bold());
}
