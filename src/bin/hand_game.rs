use rand::Rng;
use std::io;
extern crate rand;

#[derive(Debug)]
enum Winner {
    User,
    System,
    Tie,
}

#[derive(Debug)]
enum Choice {
    Stone,
    Paper,
    Scissor,
}

#[derive(Debug)]
struct WinCount {
    user: u32,
    system: u32,
    tie: u32,
}

fn decide_winner(user_choice: &Choice, system_choice: &Choice) -> Winner {
    match (user_choice, system_choice) {
        (Choice::Stone, Choice::Scissor) => Winner::User,
        (Choice::Paper, Choice::Stone) => Winner::User,
        (Choice::Scissor, Choice::Paper) => Winner::User,
        (Choice::Stone, Choice::Paper) => Winner::System,
        (Choice::Paper, Choice::Scissor) => Winner::System,
        (Choice::Scissor, Choice::Stone) => Winner::System,
        _ => Winner::Tie,
    }
}

/// Generate system's choice randomly.
fn get_system_choice() -> Choice {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..3) {
        0 => Choice::Stone,
        1 => Choice::Paper,
        _ => Choice::Scissor,
    }
}

fn get_user_choice() -> Choice {
    println!("Enter your choice (stone/paper/scissor):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read!");

    match input.trim().to_lowercase().as_str() {
        "stone" => Choice::Stone,
        "paper" => Choice::Paper,
        "scissor" => Choice::Scissor,
        _ => {
            println!("Invalid choice. Please pass 'stone', 'paper', or 'scissor'.");
            get_user_choice()
        }
    }
}

fn print_winner(winner: &Winner) {
    match winner {
        Winner::User => println!("👨 You wins this time!"),
        Winner::System => println!("🤖 System wins this time!"),
        Winner::Tie => println!("🫂 It's a Tie!"),
    }
}

fn ask_playing_chances() -> u32 {
    let mut times = String::new();
    println!("How many times you want to play the game?");
    io::stdin().read_line(&mut times).expect("Failed to read!");
    match times.trim().parse() {
        Ok(x) => x,
        Err(err) => {
            println!("❌ {}", err);
            ask_playing_chances()
        }
    }
}

fn main() {
    println!("Welcome to Stone, Paper, Scissor Game!");

    let mut win_count = WinCount {
        user: 0,
        system: 0,
        tie: 0,
    };

    let mut chances = ask_playing_chances();
    while chances != 0 {
        let user_choice = get_user_choice();
        let system_choice = get_system_choice();

        println!(
            "👨 Your Choice: {:?}\n🤖 System's choice: {:?}",
            user_choice, system_choice
        );

        let winner = decide_winner(&user_choice, &system_choice);
        match winner {
            Winner::User => win_count.user += 1,
            Winner::System => win_count.system += 1,
            Winner::Tie => win_count.tie += 1,
        };
        print_winner(&winner);
        chances -= 1;
    }

    println!("\nFinal result: {:?}", win_count);
}
