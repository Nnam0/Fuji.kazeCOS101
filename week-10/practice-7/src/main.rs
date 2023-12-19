// Declare a structure
struct Employee {
    ceo: String,
    company: String,
    age: u32,
}

// Function to display employee information
fn display(emp: Employee) {
    println!(
        "CEO: {}, Company: {}, Age: {}",
        emp.ceo, emp.company, emp.age
    );
}

fn main() {
    // Initialize structures
    let emp1 = Employee {
        company: String::from("Microsoft Corporation"),
        ceo: String::from("Satya Nadella"),
        age: 56,
    };

    let emp2 = Employee {
        company: String::from("Google Inc."),
        ceo: String::from("Sundar Pichai"),
        age: 51,
    };

    // Pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}
