// Rust program to input the age and experience of an employee
// and their annual incentive
// print if someone is experienced, unexperienced and the age
use std::io;

fn main()
 {
    let mut input1 = String::new();

    println!("\nEnter Your Age");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f32 = input1.trim().parse().expect("Not a valid string");

    if age >= 40.00
    {
        println!("You are experienced and your incentive is 1,560,000.00");
    }
    else if age > 30.00 && age <= 40.00
    {
        println!("You are experienced and your incentive is 1,480,000.00");
    }
    else if age > 20.00  && age <=30.00
    {
        println!("You are experienced and your incentive is 1,300,000.00");
    }
    else
    {
        println!("You are inexperienced and your incentive is 100,000.00");
    }

}


