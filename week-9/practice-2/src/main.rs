use std::io::Read; // Fix the import statement

fn main() {
    let mut file = std::fs::File::open("welcome_message.txt").unwrap(); // Fix the assignment operator
    let mut contents = String::new(); // Fix the assignment operator
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
