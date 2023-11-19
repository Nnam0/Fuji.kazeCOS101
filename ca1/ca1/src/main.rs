use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("Health Diagnosis list: ");
    println!("Alz = Alzheimer = 1200000");
    println!("Arr = Arrthymia = 550000");
    println!("Ckd = Chronic kidney disease = 1500000");
    println!("Dbs = Diabetes = 800000");
    println!("Arh = Arthritis = 450000");

    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();
    let mut input11 = String::new();
    let mut input12 = String::new();
    let mut input13 = String::new();



    println!("Enter Your Name");
    io::stdin().read_line(&mut input6).expect("Not a valid string");

    println!("Enter your Date of Birth: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    
    println!("Enter your email address: ");
    io::stdin().read_line(&mut input8).expect("Not a valid string");

    println!("Enter your phone number: ");
    io::stdin().read_line(&mut input9).expect("Not a valid string");

    println!("Enter your number of siblings: ");
    io::stdin().read_line(&mut input10).expect("Not a valid string");
    let no of sibling:i32 = input10.trim().parse().expect("Not a valid number");

    println!("Enter your number of children: ");
    io::stdin().read_line(&mut input11).expect("Not a valid string");
    let no of children:i32 = input11.trim().parse().expect("Not a valid number");

    println!("Enter your medical diagnosis: ");
    io::stdin().read_line(&mut input12).expect("Not a valid string");

    println!("Enter your village of residence: ");
    io::stdin().read_line(&mut input13).expect("Not a valid string");

     if patient has alz
     let total = 1200000.0   
     if age >= 40.00
     if no of children = 4.0
     let discount = 0.20 * total
      let discounted_total = total - discount
    println!("20% Discount Applied! Discounted Total: N{:.2}", discounted_total);
    } else {
        println!("Total Amount: N{:.2}",total);

     
}
    
    


    






    

