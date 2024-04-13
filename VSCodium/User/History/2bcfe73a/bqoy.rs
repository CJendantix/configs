fn main() {
    let a = 5;
    let b = 2;
    let mut output: i32 = a;
    for _ in 0..b {
        output *= b
    }
    println!("{}", output);
}
