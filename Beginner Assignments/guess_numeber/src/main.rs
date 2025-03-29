use colored::Colorize;
use rand::Rng;
use std::io::{self, Write};

fn main() {
    print!("please enter your name? ");
    io::stdout().flush().expect("failed to flush out");
    let mut username: String = String::new();
    std::io::stdin()
        .read_line(&mut username)
        .expect("Error in input!!");
    println!(
        "{}",
        format!(
            "=============== Hi **{}** ===============",
            username.trim().green()
        )
    );

    // set the level of game
    let s = format!(
        "Easy ==> 1 \nModerate ==> 2 \nHard ==> 3 \nExit => 4\nPlease choose the level of game(1,2,3)?"
    );

    println!("{s}");
    io::stdout().flush().expect("failed to flush out");
    let mut level: String = String::new();
    std::io::stdin()
        .read_line(&mut level)
        .expect("Error in input!!");
    let level: i32 = match level.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input! Please enter a valid number for the level.");
            return;
        }
    };

    //set the number of blood based of level of game
    let guess_count = match level {
        1 => 8,
        2 => 7,
        3 => 6,
        _ => {
            println!("{}", "**Hope to see you again**".green());
            return;
        }
    };

    if level == 1 || level == 2 || level == 3 {
        let mut user_guess: String = String::new();
        //receive guess for first time and loop again if it is not in valid format
        let mut blood = blood_initialize(guess_count);

        loop {
            user_guess.clear();
            print!("please guess a number  => ");
            io::stdout().flush().expect("failed to flush out");
            std::io::stdin()
                .read_line(&mut user_guess)
                .expect("Error in input!!");

            if valid_number(&user_guess.to_string()) {
                user_guess = user_guess.trim().to_string();
                blood_reduction(&mut blood);
                break;
            } else {
                println!(
                    "please provide a number with not repeated digit and not start with zero!!!! guess again."
                )
            }
        }
        //receive random guess in valid format
        let comp_guess = random_guess(level);

        loop {
            if guess_evaluation(&user_guess, &comp_guess, &mut blood) == true {
                println!("{}", "congrats!!!!".blue());
                break;
            } else {
                if blood.is_empty() {
                    println!("{}", "Game Over!!!!".red());
                    break;
                }
                loop {
                    print!("new guess => ");
                    io::stdout().flush().expect("failed to flush out");

                    user_guess.clear();
                    std::io::stdin()
                        .read_line(&mut user_guess)
                        .expect("Error in input!!");

                    if valid_number(&user_guess.trim().to_string()) {
                        user_guess = user_guess.trim().to_string();
                        blood_reduction(&mut blood);
                        break;
                    } else {
                        println!(
                            "please provide a number with not repeated digit and not start with zero!!!! guess again."
                        );
                    }
                }
            }
        }
    } else {
        println!("{}", "**Hope to see you again**".green());
        return;
    }
}

/*
 * random guess : if the level is 1 then the random guess is in range of [1,999]
 * and if the level is 2 then the random guess is in range of [1,9999]
 * otherwise for hard level it is in range of [1,99999]
 */
fn random_guess(level: i32) -> String {
    let mut rng = rand::rng();

    loop {
        let ran_gen: i32 = match level {
            // note that if the number should have 3 digits and it cannot stats with zero and should not have repeated digits, therefore, first number is 102!
            1 => rng.random_range(102..1000),
            2 => rng.random_range(1023..10000),
            _ => rng.random_range(10234..100000),
        };
        if valid_number(&ran_gen.to_string()) {
            return ran_gen.to_string();
        }
    }
}

//compare user and random guess and return a true or false based on comparison
fn guess_evaluation(user_guess: &String, comp_guess: &String, blood: &mut String) -> bool {
    blood_reduction(blood); // each time you are evaluate user and comp guess you will lose blood by one
    let user_array: Vec<char> = user_guess.chars().collect();
    let comp_array: Vec<char> = comp_guess.chars().collect();

    if user_array.len() != comp_array.len() {
        println!("Error: Lengths of user_guess and comp_guess do not match!");
        return false;
    }

    let mut guard = true;
    for (user_char, comp_char) in user_array.iter().zip(comp_array.iter()) {
        if user_char == comp_char {
            print!("{}", user_char.to_string().green());
        } else if comp_guess.contains(*user_char) {
            print!("{}", user_char.to_string().yellow());
            guard = false;
        } else {
            print!("{}", user_char.to_string().red());
            guard = false;
        }
    }
    print!("   {}", blood.red());
    println!();
    guard
}

/*
 * check the validity of number provided by user or randomly generated by computer
 * the number cannot start with 0 and also repeated digits are not allowed
 */

fn valid_number(number: &String) -> bool {
    if number.starts_with("0") {
        false
    } else {
        //check for repeated digits
        for ch in number.chars() {
            if number.chars().filter(|&c| c == ch).count() >= 2 {
                return false;
            }
        }
        true
    }
}
//initialize blood symbols based on the level of game
fn blood_initialize(count: i32) -> String {
    let mut s: String = String::new();
    for _ in 0..=count {
        s.push_str("\u{2588}");
    }
    s
}
//reduction of blood symbol by one
fn blood_reduction(s: &mut String) {
    s.pop();
}
