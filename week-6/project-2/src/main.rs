use std::io;

fn main() {
  
        for _ in 0..500{

        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Enter name: ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");


        println!("Enter number of papers published: ");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let no_of_papers:i32 = input2.trim().parse().expect("Not a valid number");

        if no_of_papers >= 3 && no_of_papers <= 5 {
            println!("Name of paper: {}The incentive is N500,000",input1);
        } if no_of_papers > 5 && no_of_papers < 10 {
            println!("Name of paper: {}The incentive is N800,000",input1);
        }  if no_of_papers >= 10 {
            println!("Name of paper: {}The incentive is N1,000,000",input1);
        }  if no_of_papers > 3  {
            println!("Name of paper: {}The incentive is N100,000",input1);
        }    
            
        }    





}
