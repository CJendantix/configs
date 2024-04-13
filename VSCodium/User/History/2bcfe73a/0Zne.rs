
mod point {
    use std::ops::{Add, Sub, Mul, Div};

    struct Point {
        x: i64,
        y: i64,
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
}

fn main() {

}