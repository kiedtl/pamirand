fn rand_parkmiller(state: u64) -> u64 {
    return (state * 48271) % 0x7fffffff;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    let state: u64 = arg.parse::<u64>().unwrap();
    println!("{}", rand_parkmiller(state));
}
