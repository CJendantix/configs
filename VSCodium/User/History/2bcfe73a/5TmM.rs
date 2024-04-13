fn main() {
    let a = 5;
    let b = 2;
    let mut output = a;
    for _ in 1..b {
        output *= a
    }
    println!("{}", output);
}
