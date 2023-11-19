use std::io;

fn main() {
   println!("PAN ATLANTIC UNIVERSITY REGISTRY");

   for _ in 0..150 {

        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut c = String::new();
        let mut l = String::new();
        let mut input7 = String::new();


        println!("Enter Your Name");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let name = input1.trim();

        println!("Enter Your Email");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let student_email = input2.trim();

        println!("Enter Your Department");
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let department = input3.trim();

        println!("Enter Your State of Origin");
        io::stdin().read_line(&mut input4).expect("Not a valid string");
        let state = input4.trim();


        println!("Are you a class rep?");
        io::stdin().read_line(&mut c).expect("Not a valid string");
        let class_rep:bool = c.trim().parse().expect("Not valid");

        println!("Enter Your Class Year");
        io::stdin().read_line(&mut l).expect("Not a valid string");
        let ll:i32 = l.trim().parse().expect("Not a valid number");

        println!("Enter Your CGPA");
        io::stdin().read_line(&mut input7).expect("Not a valid string");
        let g:f32 = input7.trim().parse().expect("Not a valid number");




        if class_rep == true && ll > 100 && g > 4.0{
            println!("Name: {}", name);
            println!("You are eligible to vote");
        }
        else {
            println!("You are not eligible to vote");
        }
    }
    
}

