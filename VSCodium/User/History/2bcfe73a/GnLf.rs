fn main() {
    /*
        tetration 3 to 2
        = 2 ^ 2 ^ 2 = (2 * 2) * (2 * 2)
        = 4 ^ 2 = 4 * (2 * 2)
        = 16 = 4 * 4
    */
    let base = 2;
    let tetration = 3;
    let mut output: i32 = base;
    for _ in 0..tetration {
        output = output.pow(base)
    }
}
