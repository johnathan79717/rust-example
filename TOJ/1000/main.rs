use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).expect("");

  let mut it = input.split_whitespace();
  //println!("{}", it.next().unwrap());
  //println!("{}", it.next().unwrap());
  let a: i32 = it.next().unwrap().parse().unwrap();
  let b: i32 = it.next().unwrap().parse().unwrap();

  println!("{}", a+b);
}
