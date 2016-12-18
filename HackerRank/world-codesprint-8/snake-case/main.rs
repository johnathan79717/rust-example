use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    let v: Vec<_> = input.split('_').collect();
    println!("{}", v.len());
}
