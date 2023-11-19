use std::io;

fn main() {
    println!("Hello, world!");
    let mut input1 = String::new();

    println!("input the number you want to see");
    io::stdin().read_line(&mut input1).expect("not an integer");
    let n:u64 = input1.trim().parse().expect("not an integer");

    println!("");

    for a in 1..n+1 {
        for b in 1..n+1 {
            print!("{}\t", a*b);
        }
        println!("");
    }
}

