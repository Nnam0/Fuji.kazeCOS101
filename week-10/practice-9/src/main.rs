fn main() {
    let v = vec![20, 40, 60, 80];
    // vector v owns the object in the heap

    let v2 = v.clone(); // Clone the vector to create a new ownership
    let v2_return = display(v2);
    println!("In main {:?}", v);
}

fn display(v: Vec<i32>) -> Vec<i32> {
    // returning the same vector
    println!("inside display {:?}", v);
    return v;
}
