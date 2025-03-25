fn main() {
    /*
     *match Scores calculation
     */
    let mut games_number = String::new();
    println!("please provide a number for the total games?");

    std::io::stdin()
        .read_line(&mut games_number)
        .expect("Error");
    let mut games_number: i32 = games_number.trim().parse().expect("error");
    let mut score_a = 0;
    let mut score_b = 0;
    let mut a: i32;
    let mut b: i32;

    while games_number > 0 {
        let mut result = String::new();
        println!("match result => ?"); // for example if its a win game enter 3 1 
        std::io::stdin().read_line(&mut result).expect("Error");
        let mut results = result.split(' ');
        a = (results.next().expect("error"))
            .trim()
            .parse()
            .expect("error");
        b = (results.next().expect("error"))
            .trim()
            .parse()
            .expect("error");
        if a == b {
            score_a += 1;
            score_b += 1;
        } else if a > b {
            score_a += 3;
        } else {
            score_b += 3;
        }
        games_number -= 1;
    }
    println!("the score of team a is => {}", score_a);
    println!("the score of team b is => {}", score_b);
}
