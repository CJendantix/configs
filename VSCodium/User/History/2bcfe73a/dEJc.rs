use std::ops::Add;

struct Point {
    x: i64,
    y: i64,
}

impl Add for Point {
    type Output = Self;
}

fn main() {
    todo!()
}