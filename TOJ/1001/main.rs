use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("");
    for a in input.split_whitespace().rev() {
        println!("{}", a.parse::<f64>().unwrap().sqrt());
    }
}
