fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error");
    let mut digits = input.split(' ');
    let first: i32 = (digits.next().expect("error"))
        .trim()
        .parse()
        .expect("error");
    let second: i32 = (digits.next().expect("error"))
        .trim()
        .parse()
        .expect("error");
    let third: i32 = (digits.next().expect("error"))
        .trim()
        .parse()
        .expect("error");
    if first > 0 && second > 0 && third > 0 && (first + second + third == 180) {
        print!("Yes")
    } else {
        println!("No");
    }
}
