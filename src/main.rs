fn rand_parkmiller(state: u64) -> u64 {
    return (state * 48271) % 0x7fffffff;
}
fn main() {
    println!("Hello, world!");
}
