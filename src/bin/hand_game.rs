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
fn generate_system_choice() -> Choice {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..3) {
        0 => Choice::Stone,
        1 => Choice::Paper,
        _ => Choice::Scissor,
    }
}

fn take_user_choice() -> Result<Choice, String> {
    println!("Enter your choice (stone/paper/scissor):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read line".to_string())?;

    match input.trim().to_lowercase().as_str() {
        "stone" => Ok(Choice::Stone),
        "paper" => Ok(Choice::Paper),
        "scissor" => Ok(Choice::Scissor),
        _ => Err("Invalid choice. Please choose 'stone', 'paper', or 'scissor'.".to_string()),
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
    let chances: u32 = times.trim().parse().expect("Please enter a valid integer.");
    return chances;
}

fn main() {
    println!("Welcome to Stone, Paper, Scissor Game!");

    let mut win_cont = WinCount {
        user: 0,
        system: 0,
        tie: 0,
    };

    let mut chances = ask_playing_chances();
    while chances != 0 {
        let user_choice = take_user_choice();
        let system_choice = generate_system_choice();

        match user_choice {
            Ok(user_choice) => {
                println!(
                    "👨 Your Choice: {:?}\n🤖 System's choice: {:?}",
                    user_choice, system_choice
                );

                let winner = decide_winner(&user_choice, &system_choice);
                match winner {
                    Winner::User => win_cont.user += 1,
                    Winner::System => win_cont.system += 1,
                    Winner::Tie => win_cont.tie += 1,
                };
                print_winner(&winner);
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        chances -= 1;
    }

    println!("\nFinal result: {:?}", win_cont);
}
