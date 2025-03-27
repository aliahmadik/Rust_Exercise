/*
 * https://pynative.com/python-basic-exercise-for-beginners/
 * Exercise 2: Print the Sum of a Current Number and a Previous number
 * Write a Python code to iterate the first 10 numbers, and in each iteration,
 * print the sum of the current and previous number.
 */
fn main() {
    println!("please provide a number for iteration counts? ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Error in input");
    let n: i32 = n.trim().parse().expect("error in type casting");
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    println!("Printing current and pervious number sum in a range({})", n);
    while i < n {
        println!(
            "Current number {} Pervious number {} Sum: {}",
            i,
            j,
            (i + j)
        );
        j = i;
        i += 1;
    }
}
