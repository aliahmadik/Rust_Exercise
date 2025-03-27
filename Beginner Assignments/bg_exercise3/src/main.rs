/*
 * Exercise 3: Print characters present at an even index number
 * Write a Rust code to accept a string from the user
 * and display characters present at an even index number.
 */
fn main() {
    println!("please provide a string = ");
    let mut input = String::new();
    //read string from input
    std::io::stdin()
        .read_line(&mut input)
        .expect("error in input string");
    //iteration over string using enumerate function
    for (index, ch) in input.trim().chars().enumerate() {
        if index % 2 == 0 {
            println!("{}", ch);
        }
    }
}
