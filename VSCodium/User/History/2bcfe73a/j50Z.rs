fn main() {
    /*
        tetration 3 to 2
        = 2 ^ 2 ^ 2 = (2 * 2) * (2 * 2)
        = 4 ^ 2 = 4 * (2 * 2)
        = 16 = 4 * 4
    */
    let base: u32 = 2;
    let tetration: u32 = 3;
    let mut output: u32 = base;
    for _ in 0..tetration - 1 {
        for _ in 0..base {
            output = output * base
        }
    }
    println!("{}", output);
}
