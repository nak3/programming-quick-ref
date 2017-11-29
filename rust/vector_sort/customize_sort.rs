use std::io::*;
use std::str::*;

const MOD: i32 = 1000000007;

fn read<T: FromStr>() -> Option<T> {
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok()
}

fn main() {
    let n: usize = read().unwrap();
    let mut v: Vec<String> = Vec::new();
    for _ in 0..n {
        let tmp = read().unwrap();
        v.push(tmp);
    }
    v.sort_by(|l, r| if l.len() == r.len() {
        l.cmp(r)
    } else {
        l.len().cmp(&r.len())
    });

    for i in 0..n {
        println!("{}", v[i]);
    }
}

/*
6
31415926535897932384626433832795
1
3
11
3
5
* */
