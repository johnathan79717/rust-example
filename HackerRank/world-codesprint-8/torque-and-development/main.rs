use std::io::stdin;
use std::collections::LinkedList;

fn query() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    let line: Vec<_> = input.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let n = line[0];
    let m = line[1];
    let c_lib = line[2];
    let c_road = line[3];

    let mut adj: Vec<LinkedList<usize>> = vec![LinkedList::<usize>::new(); n];
    for _ in 0..m {
        input.clear();
        stdin().read_line(&mut input);
        let line: Vec<_> = input.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let a = line[0] - 1;
        let b = line[1] - 1;
        adj[a].push_back(b);
        adj[b].push_back(a);
    }

    if c_lib <= c_road {
        println!("{}", c_lib * n);
        return;
    }

    let mut visited: Vec<bool> = vec![false; n];
    fn dfs(v: usize, adj: &Vec<LinkedList<usize>>, visited: &mut Vec<bool>) {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                dfs(u, adj, visited);
            }
        }
    };
    let mut components = 0;
    for v in 0..n {
        if !visited[v] {
            components += 1;
            visited[v] = true;
            dfs(v, &adj, &mut visited);
        }
    }

    println!("{}", c_lib * components + c_road * (n - components));
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    let q: i32 = input.trim().parse().unwrap();
    for _ in 0..q {
        query();
    }
}
