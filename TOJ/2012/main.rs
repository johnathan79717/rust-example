use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    println!("{}",
             if input.trim().parse::<u8>().unwrap() >= 7 {
                 "YES"
             } else {
                 "NO"
             });
}
