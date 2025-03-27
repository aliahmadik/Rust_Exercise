fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input);
    let mut t: i32 = input.trim().parse().expect("error");
    if t >= -273 && t < 6000 {
        let result: String;
        if t > 100 {
            result = "Steam".to_string();
        } else if t < 0 {
            result = "Ice".to_string();
        } else {
            result = "Water".to_string();
        }
        println!("{result}")
    } else {
        println!("the T you entered is wrong");
    }
}
