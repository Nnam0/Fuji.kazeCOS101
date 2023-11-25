use std::io;

fn main() {
    let mut input = String::new();
    let mut input1 = String::new();
    let mut input2= String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();
    let mut input11 = String::new();
    let mut input12 = String::new();
    let mut input13 = String::new();
    let mut input14 = String::new();
    let mut input15 = String::new();
    let mut input16 = String::new();
    println!("how many siblings do you have");
    io::stdin().read_line(&mut input).expect("input a digit");
    let nom:i64 = input.trim().parse().expect("not an integer");

    for _ in 0..nom{

        println!("enter name of sibling");
        io::stdin().read_line(&mut input).expect("can't read character");
        let name = input1.trim();

        println!("enter age of sibling");
        io::stdin().read_line(&mut input2).expect("can't read character");
        let age:i64 = input2.trim().parse().expect("can't read character");

        if age > 18 {
            println!("what is your marital status??(married/single)");
            io::stdin().read_line(&mut input3).expect("not a character");
            let marital_status = input3.trim();

            if marital_status == "single"{
                println!("what is employment status??(employed/student)");
                io::stdin().read_line(&mut input4).expect("invalid");
                let employment_status = input4.trim();

                if employment_status == "student" {
                    println!("what university does he/she attend??");
                    io::stdin().read_line(&mut input5).expect("invalid");
                    let university = input5.trim();

                    println!("what course is he/she studying??");
                    io::stdin().read_line(&mut input6).expect("invalid");
                    let course = input6.trim();
                }else if employment_status == "employed"{
                    println!("where does he/she work??");
                    io::stdin().read_line(&mut input11).expect("invalid");
                    let work_place = input11.trim();
                }
            }else if marital_status == "married"{
                println!("does he/she have kids??");
                io::stdin().read_line(&mut input7).expect("invalid");
                let any_kids = input7.trim();

                println!("where do they base with their family??");
                io::stdin().read_line(&mut input7).expect("invalid");
                let base = input8.trim();
            }else {
                println!("not part of requirements");
            }
        }else if age < 18 {
            println!("has he/she written WAEC??(yes/no)");
            io::stdin().read_line(&mut input8).expect("invalid");
            let waec_status = input8.trim();

            if waec_status == "yes" {
                println!("enter name of secondary school attended");
                io::stdin().read_line(&mut input9).expect("invalid");
                let secondary_school = input9.trim();
            }if waec_status == "no" {
                println!("what class is he/she in now??");
                io::stdin().read_line(&mut input10).expect("invalid");
                let class = input10.trim();
            }
        }
    }
}
