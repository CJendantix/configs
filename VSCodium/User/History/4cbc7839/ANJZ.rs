use rand::{random, thread_rng, Rng};
fn main() {
    let mut rng = thread_rng();
    println!("{}", {if rng.gen_range(1..=2) == 1 {"a"} else {"b"}});
}