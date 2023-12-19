fn main() {
    let v = vec![10, 20, 30];
    let v2 = v; // Ownership transfer

    display(v2.clone()); // Pass a clone of v2 to the display function

    // v2 is No longer usable here
    println!("In main {:?}", v2);
}

fn display(v: Vec<i32>) {
    println!("Inside display {:?}", v);
}
