fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error");
    //note that the first digit is seat number and second is row number.
    let mut words = input.split(' ');
    let c: i32 = (words.next().expect("error")) //seat number
        .trim()
        .parse()
        .expect("error");
    let r: i32 = (words.next().expect("error")) // row number
        .trim()
        .parse()
        .expect("error");

    if c <= 10 {
        print!("Right {} {}", 11 - r, c);
    } else {
        print!("Left {} {}", 11 - r, 21 - c);
    }
}
