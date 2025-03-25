/*
 * Exercise 1: Calculate the multiplication and sum of two numbers
 * Given two integer numbers, write a Rust code to return their product only if the product is equal to or lower than 1000. Otherwise, return their sum
 * Ahmadikatouli@gmail.com
 */
fn main() {
    let mut number1: String = String::new();
    let mut number2: String = String::new();
    println!("please enter number 1");
    std::io::stdin()
        .read_line(&mut number1)
        .expect("Error in input");
    println!("please enter number 2");
    std::io::stdin()
        .read_line(&mut number2)
        .expect("Error in input");
    let number1: i32 = number1.trim().parse().expect("Error in type casting");
    let number2: i32 = number2.trim().parse().expect("Error in type casting");
    if number1 * number2 <= 1000 {
        println!(
            "the multiplication of {} * {} = {}",
            number1,
            number2,
            number1 * number2
        );
    } else {
        println!(
            "the addition of {} + {} = {}",
            number1,
            number2,
            number1 + number2
        );
    }
}
