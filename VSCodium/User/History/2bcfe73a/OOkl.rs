
mod point {
    use std::ops::{Add, Sub, Mul, Div};
    use std::fmt::Display;

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Point {
            Point {
                x,
                y,
            }
        }
    }

    impl Add for Point {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Sub for Point {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl Mul for Point {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }

    impl Div for Point {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            }
        }
    }

    impl Display for Point {
        
    }
}

use point::*;
fn main() {
    let point_1 = Point::new(5.0, 7.0);
    let point_2 = Point::new(8.0, 11.0);

    println!("{:?} {:?} {:?} {:?}", point_1+point_2, point_1-point_2,point_1*point_2,point_1/point_2)
}