fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error");
    let mut n: i32 = input.trim().parse().expect("error");
    print!("W");
    while n > 0 {
        print!("o");
        n -= 1;
    }
    print!("w!");
}
