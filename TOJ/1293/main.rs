use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let mut ans = 2;
    for a in input.split_whitespace() {
        ans *= a.parse().unwrap();
    }
    println!("{}", ans);
}
