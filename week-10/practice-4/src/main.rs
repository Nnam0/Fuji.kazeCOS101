fn main() {
    // a list of numbers
    let x = vec![100, 200, 300];
    borrow_vector(&x); // passing a reference

    // Use &x[0] to borrow the value from the vector
    println!("Printing the value from main() x[0]={}", &x[0]);
    println!("****************************");
}

fn borrow_vector(z: &Vec<i32>) {
    println!("************************");
    println!("Inside borrow_vector function {:?} \n", z);
    println!("-------------------------");
}
