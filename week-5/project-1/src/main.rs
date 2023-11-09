use std::io;

fn main() {


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter the coefficient a of the quadratic equation:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Not a valid string");

    println!("Enter the coefficient b of the quadratic equation:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Not a valid string");

    println!("Enter the coefficient c of the quadratic equation:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Not a valid string");


    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let sqrt_discriminant = discriminant.sqrt();
        let root1 = (-b + sqrt_discriminant) / (2.0 * a);
        let root2 = (-b - sqrt_discriminant) / (2.0 * a);
        println!("Two real roots: Root1 = {}, Root2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: Root = {}", root);
    } else {
        println!("No real roots (complex roots)");
    }
}


 
    

