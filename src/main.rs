use std::env;

fn rand_parkmiller(state: u64) -> u64 {
    return (state * 48271) % 0x7fffffff;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pamirand v1.0.0");
        println!("MIT (c) LPTSTR 2019");
        println!("Usage: pamirand [seed]");
        return;
    }
    let arg = &args[1];
    let state: u64 = arg.parse::<u64>().unwrap();
    println!("{}", rand_parkmiller(state));
}
