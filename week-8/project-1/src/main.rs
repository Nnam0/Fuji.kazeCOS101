use std::io;

fn main() {

 let public_servant = vec!["Office Administrator","Academic","Lawyer",
 "Teacher"]; 
 let mut input1 = String::new();
 let mut input2 = String::new();
 let mut input3 = String::new();

 println!("Welcome to the Public Service APS Level Checker For The FGoN");
 io::stdin().read_line(&mut input1).expect("Not a valid string");

 println!("We only Check levels for the following:
           {:?}
           {:?}
           {:?}
           {:?}",public_servant[0],public_servant[1],public_servant[2],
           public_servant[3]);

println!("How many Staff are checking their levels today?");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let staff_number:i32 = input2.trim().parse().expect("Not a valid Integer");
let mut x = 0;
x+=1;
loop{

    println!("Enter 1 for {:?}
              Enter 2 for {:?}
              Enter 3 for {:?}
              Enter 4 for {:?}",public_servant[0],public_servant[1],public_servant[2],
           public_servant[3]);

    println!("What kind of public sevant are you?");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let ps:i32 = input3.trim().parse().expect("Not a valid Integer");
    if ps == 1
    {
        office_administrator();
    }
    else if ps == 2
    {
        academic();
    }
    else if ps == 3
    {
        lawyer();
    }
    else if ps == 4
    {
        teacher();
    }
    else 
    {
        println!("We don't check for this public service");
    }  
    if x == staff_number
    {
    break;
    } 
    }

    println!("Thank you for using our service :)");
}

fn office_administrator(){
    let mut input1 = String::new();

    let office_admininstrator = vec!["Intern","Administrator",
    "Senior administrator","Office Manager","Director","CEO"];

    let level = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10",
    "EL2 10-13", "SES"];

    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let experience:i32 = input1.trim().parse().expect("Not a valid integer");


    if experience == 1 || experience == 2 
    {
        println!("You are an {} and you are at the level {}"
            ,office_admininstrator[0],level[0] );
    }
    else if experience >=3 && experience <= 5
    {
        println!("You are an {} and you are at the level {}"
            ,office_admininstrator[1],level[1]);
    }
    else if experience >=5 && experience <= 8
    {
        println!("You are a {} and you are at the level {}"
            ,office_admininstrator[2],level[2]);
    }
    else if experience >=8 && experience <= 10
    {
        println!("You are an {} and you are at the level {}"
            ,office_admininstrator[3],level[3]);
    }
    else if experience >=10 && experience <= 13
    {
        println!("You are a {} and you are at the level {}"
            ,office_admininstrator[4],level[4]);
    }
    else if experience <= 13
    {
        println!("You are a {} and you are at the level {}"
            ,office_admininstrator[5],level[5]);
    }
    else 
    {
        println!("Not a enough work experience");
    }
}
fn academic(){
    let mut input1 = String::new();

    let academic = vec!["N/A","Research Assistant","PhD Candidate",
    "Post-Doc Researcher","Senior lecturer","Dean"];

    let level = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10",
    "EL2 10-13", "SES"];

    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let experience:i32 = input1.trim().parse().expect("Not a valid integer");


    if experience == 1 || experience == 2 
    {
        println!("{} and you are at the level {}"
            ,academic[0],level[0] );
    }
    else if experience >=3 && experience <= 5
    {
        println!("You are a {} and you are at the level {}"
            ,academic[1],level[1]);
    }
    else if experience >=5 && experience <= 8
    {
        println!("You are a {} and you are at the level {}"
            ,academic[2],level[2]);
    }
    else if experience >=8 && experience <= 10
    {
        println!("You are a {} and you are at the level {}"
            ,academic[3],level[3]);
    }
    else if experience >=10 && experience <= 13
    {
        println!("You are a {} and you are at the level {}"
            ,academic[4],level[4]);
    }
    else if experience <= 13
    {
        println!("You are a {} and you are at the level {}"
            ,academic[5],level[5]);
    }
    else 
    {
        println!("Not a enough work experience");
    }
}
fn lawyer(){
    let mut input1 = String::new();

    let lawyer = vec!["Paralegal","Junior Associate","Associate",
    "Senior Associate 1-2","Senior Associate 3-4","Partner"];

    let level = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10",
    "EL2 10-13", "SES"];

    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let experience:i32 = input1.trim().parse().expect("Not a valid integer");


    if experience == 1 || experience == 2 
    {
        println!("You are a {} and you are at the level {}"
            ,lawyer[0],level[0] );
    }
    else if experience >=3 && experience <= 5
    {
        println!("You are a {} and you are at the level {}"
            ,lawyer[1],level[1]);
    }
    else if experience >=5 && experience <= 8
    {
        println!("You are an {} and you are at the level {}"
            ,lawyer[2],level[2]);
    }
    else if experience >=8 && experience <= 10
    {
        println!("You are a {} and you are at the level {}"
            ,lawyer[3],level[3]);
    }
    else if experience >=10 && experience <= 13
    {
        println!("You are a {} and you are at the level {}"
            ,lawyer[4],level[4]);
    }
    else if experience <= 13
    {
        println!("You are a {} and you are at the level {}"
            ,lawyer[5],level[5]);
    }
    else 
    {
        println!("Not a enough work experience");
    }
}
fn teacher(){
    let mut input1 = String::new();

    let teacher = vec!["Placement","Classroom teacher","Snr teacher",
    "Leading teacher","Deputy principal","Principal"];

    let level = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10",
    "EL2 10-13", "SES"];

    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let experience:i32 = input1.trim().parse().expect("Not a valid integer");


    if experience == 1 || experience == 2 
    {
        println!("You are a {} and you are at the level {}"
            ,teacher[0],level[0] );
    }
    else if experience >=3 && experience <= 5
    {
        println!("You are a {} and you are at the level {}"
            ,teacher[1],level[1]);
    }
    else if experience >=5 && experience <= 8
    {
        println!("You are a {} and you are at the level {}"
            ,teacher[2],level[2]);
    }
    else if experience >=8 && experience <= 10
    {
        println!("You are a {} and you are at the level {}"
            ,teacher[3],level[3]);
    }
    else if experience >=10 && experience <= 13
    {
        println!("You are a {} and you are at the level {}"
            ,teacher[4],level[4]);
    }
    else if experience <= 13
    {
        println!("You are a {} and you are at the level {}"
            ,teacher[5],level[5]);
    }
    else 
    {
        println!("Not a enough work experience");
    }
}        
    }

