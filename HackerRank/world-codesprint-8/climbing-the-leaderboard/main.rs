use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    input.clear();
    stdin().read_line(&mut input);
    let mut scores: Vec<_> = input.split_whitespace()
                                  .map(|x| x.parse::<i32>().unwrap())
                                  .collect();
    scores.sort_by_key(|k| -k);
    scores.dedup();
    stdin().read_line(&mut input);
    input.clear();
    stdin().read_line(&mut input);
    let alices = input.split_whitespace()
                      .map(|x| x.parse::<i32>().unwrap())
                      .collect::<Vec<_>>();
    for x in &alices {
        let i = match scores.binary_search_by_key(&-x, |&k| -k) {
            Ok(i) => i,
            Err(i) => i,
        };
        println!("{}", i+1);
    }
}
