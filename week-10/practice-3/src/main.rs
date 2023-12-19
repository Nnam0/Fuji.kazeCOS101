fn main() {
    // a list of numbers
    let v = vec![15, 25, 35, 45, 55];
    print_vector(&v); // Pass a reference to v

    println!("{}", v[0]); // Now this line is fine
}

fn print_vector(x: &Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}
