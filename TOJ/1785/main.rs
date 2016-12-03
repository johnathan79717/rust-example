use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let ans = match input.trim().parse().unwrap() {
        1...4 => "few",
        5...9 => "several",
        10...19 => "pack",
        20...49 => "lots",
        50...99 => "horde",
        100...249 => "throng",
        250...499 => "swarm",
        500...999 => "zounds",
        _ => "legion", 
    };
    println!("{}", ans);
}
