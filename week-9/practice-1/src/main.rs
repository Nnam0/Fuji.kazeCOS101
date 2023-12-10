use std::fs::File;
use std::io::Write; // Add this import for write_all

fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science";

    let mut file = File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes())
        .expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}