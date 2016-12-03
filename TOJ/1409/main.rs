use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    for a in input.split_whitespace().rev() {
        print!("{} ", a.parse::<i32>().unwrap()-1);
    }
}
