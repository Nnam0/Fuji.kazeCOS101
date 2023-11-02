fn main() {
    let name1 = "Nnamdi Ebunam";
    println!("My name is {}",name1);

    //find and replace
    let name2 = name1.replace("Nnamdi","Nnamodi");
    println!("You can also call me {}",name2);
    let faculty = "Faculty of Science and Technology";

    //find and replace
    let school = faculty.replace("Faculty","School");
    println!("I am a student of the {}", school);
}
