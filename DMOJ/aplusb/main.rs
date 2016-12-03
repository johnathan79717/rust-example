use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).expect("");
  let n: i32 = input.trim().parse().unwrap();

  for _ in 0..n {
      let mut input = String::new();
      io::stdin().read_line(&mut input).expect("");
      let mut it = input.split_whitespace();
      let a: i32 = it.next().unwrap().parse().unwrap();
      let b: i32 = it.next().unwrap().parse().unwrap();

      println!("{}", a+b);
  }
}
