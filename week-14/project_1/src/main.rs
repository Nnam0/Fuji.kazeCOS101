use std::io::Read;
use std::io;

fn main() {
    let mut input1 = String::new();
    println!("Welcome. Please choose between the following numbers:\n1 - Administrator\n2 - Project Manager\n3 - Employee\n4 - Vendor\n5 - Customer");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let input1:i64 = input1.trim().parse().expect("Not a valid string");

    if input1 == 1 {
        open_admin()
    } else if input1 == 2{
        open_project()    
    } else if input1 == 3{
        open_staff()
    } else if input1 == 4{
        open_dataplan()
    } else if input1 == 5{
        open_customer()
    } else {
        println!("Wrong input")
    }
    
}
fn open_staff(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}
fn open_project(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents)
}
fn open_customer(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}
fn open_dataplan(){
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}
fn open_admin(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}