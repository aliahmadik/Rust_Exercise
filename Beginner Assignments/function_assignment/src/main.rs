fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error");
    let mut inputs = input.split(' ');
    let digit1: f32 = (inputs.next().expect("error"))
        .trim()
        .parse()
        .expect("error");
    let digit2: f32 = (inputs.next().expect("error"))
        .trim()
        .parse()
        .expect("error");
    let digit3: f32 = (inputs.next().expect("error"))
        .trim()
        .parse()
        .expect("error");

    println!(
        "the sum of your numbers is => {:.3}",
        add(digit1, digit2, digit3)
    );
}
// add function
fn add(num1: f32, num2: f32, num3: f32) -> f32 {
    //return sum of (num1,num2 ,num3)
    num1 + num2 + num3
}
