fn main() {
    let mut x = String::new();
    let mut y = String::new();
    println!("please provide a number for x = ");
    std::io::stdin().read_line(&mut x).expect("error in input");

    println!("please provide a number for y = ");
    std::io::stdin().read_line(&mut y).expect("error in input");

    let mut x: i32 = x.trim().parse().expect("error in type casting");
    let mut y: i32 = y.trim().parse().expect("error in type casting");

    sort(&mut x, &mut y);

    println!("x => {} y => {}", x, y);
}
fn sort(x: &mut i32, y: &mut i32) {
    if x > y {
        swap(x, y);
    }
}
fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}
