use std::io::Write;

use regex::Regex;
fn main() {
    let mut email = String::new();
    email = receive_email();
    loop {
        if email_validate(&email) {
            println!("the email: {} is valid email address.", email);
            break;
        } else {
            println!("please Provide a valid email address...");
            email = receive_email();
        }
    }
}
fn receive_email() -> String {
    print!("please provide an email address: ");
    std::io::stdout().flush().expect("error in output");
    let mut email = String::new();
    std::io::stdin()
        .read_line(&mut email)
        .expect("Error in input");
    email.trim().to_string()
}
fn email_validate(email: &String) -> bool {
    let re = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    if re.is_match(email) { true } else { false }
}
