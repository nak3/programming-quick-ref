use std::io;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).ok();

    let mut it = buf.split_whitespace().map(|n| String::from_str(n).unwrap());
    let a = it.next().unwrap();

    for c in a.chars() {
        if c == '9' {
            println!("Yes");
            return;
        }
    }
    println!("No");
    return;
}
