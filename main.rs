
use std::io;
 
const CLIENT: Client = Client::new().unwrap();



// Define the name of the collection we'll use to store high scores.
const HIGH_SCORES_COLLECTION: &str = "high_scores";

// Define the struct that will represent a player's high score in the database.
struct HighScore {
    player_name: String,
    score: i32,
}

fn main() {
    // Create a new RDBX Cloud client with your API key and secret.

    struct Client {
    }
    
    let client = Client::new("<your-api-key>", "<your-api-secret>").unwrap();

    // Get a reference to the high scores collection.
    let high_scores = client.collection(HIGH_SCORES_COLLECTION).unwrap();

    

    println!("Welcome to the guessing game!")
}

    loop {
        
    
        use rand::Rng;

let mut rng = rand::thread_rng();
let secret_number = rng.gen_range(1..=101);

        
        let mut guess_count = 0;
        let mut player_name = String::new();

        // Prompt the player for their name.
        println!("Please enter your name:");
        io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");
        player_name = player_name.trim().to_string();

        println!("Welcome, {}!", player_name);
    };

loop {
            let mut guess = String::new();

            println!("Please enter your guess (between 1 and 100):");

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number between 1 and 100.");
                    continue;
                }
            };

            guess_count += 1;

            match guess.cmp(&secret_number) {
                std::cmp::Ordering::Less => println!("Too small!"),
                std::cmp::Ordering::Greater => println!("Too big!"),
                std::cmp::Ordering::Equal => {
                    println!("You win!");
                    println!("It took you {} guesses.", guess_count);

                    // Try to retrieve the player's current high score.
                    let current_high_score = high_scores.get(&player_name);

                    if current_high_score.is_none() || current_high_score.unwrap().score > guess_count {
                        // If the player doesn't have a high score yet, or if they beat their previous high score, update the database.
                        let high_score = HighScore {
                            player_name: player_name.clone(),
                            score: guess_count,
                        };

                        high_scores.put(&high_score);

                        println!("Congratulations, {}! You have a new high score!", player_name);
                    }

                    break;
                }
            }
        };

        println!("Would you like to play again? (y/n)");

        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if !play_again.trim().eq_ignore_ascii_case("y") {
            break;
        }


