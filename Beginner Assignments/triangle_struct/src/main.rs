//struct definition
struct Triangle {
    side1: f32,
    side2: f32,
    side3: f32,
}
//Implementation of methods and functions
impl Triangle {
    //construction of a triangle --> set with default 0.0 values
    fn new() -> Triangle {
        Triangle {
            side1: 0.0,
            side2: 0.0,
            side3: 0.0,
        }
    }
    /*
     * valid triangle function : A triangle is valid if the sum of the lengths of any two sides is greater than the length of the third side.
     */
    fn valid_triangle(&self) -> bool {
        if self.side1 + self.side2 > self.side3
            && self.side1 + self.side3 > self.side2
            && self.side2 + self.side3 > self.side1
        {
            true
        } else {
            false
        }
    }
    /*
     * set the side value of triangle with given value : note that , it will change the value if values are corresponds to valid triangles
     * otherwise it will be reset with default 0.0 values
     */
    fn set_side(&mut self, v1: f32, v2: f32, v3: f32) -> Self {
        self.side1 = v1;
        self.side2 = v2;
        self.side3 = v3;
        if self.valid_triangle() {
            Self {
                side1: self.side1,
                side2: self.side2,
                side3: self.side3,
            }
        } else {
            println!("Invalid sides: Cannot form a valid triangle.");
            self.side1 = 0.0;
            self.side2 = 0.0;
            self.side3 = 0.0;
            Self {
                side1: self.side1,
                side2: self.side2,
                side3: self.side3,
            }
        }
    }
    // calculate the perimeter of a triangle
    fn perimeter(&self) -> f32 {
        self.side1 + self.side2 + self.side3
    }
    // calculate the area of a triangle
    fn area(&self) -> f32 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt()
    }
    // based on the length of sides,we have three types of triangle which are equilateral,Isosceles and scalene.
    fn type_of_triangle(&self) -> String {
        if self.side1 == self.side2 && self.side2 == self.side3 {
            String::from("Equilateral")
        } else if self.side1 == self.side2 || self.side2 == self.side3 || self.side1 == self.side3 {
            String::from("Isosceles")
        } else {
            String::from("Scalene")
        }
    }
    // print the length of each side
    fn display(&self) {
        println!(
            "the sidees of triangles => ({},{},{})",
            self.side1, self.side2, self.side3
        );
    }
}
fn main() {
    let mut triangle = Triangle::new();

    println!("Perimeter: {}", triangle.perimeter());
    println!("Area: {}", triangle.area());

    //invalid triangle set
    triangle.set_side(10.0, 12.5, 1.0);
    println!("Perimeter: {}", triangle.perimeter());

    //valid triangle set
    triangle.set_side(3.0, 5.0, 4.0);
    println!("Area: {}", triangle.area());
    //check the type of triangle
    println!("the type of triangle is => {}", triangle.type_of_triangle());
    //change the side of triangle
    triangle.side1 = 2.0;
    println!("Area: {}", triangle.area());
    //display the triangle'sides
    triangle.display();
}
