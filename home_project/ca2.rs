use std::fs::File;
use std::io::{self, Write};

struct Company {
    name: String,
    founded: String,
    shares: f32,
    liabilities: f32,
    percentage_leverages: Vec<f32>,
}

impl Company {
    fn new(name: &str, year_founded: &str, assets: f32, liabilities: f32) -> Self {
        Company {
            name: name.to_string(),
            year_founded: year_founded.to_string(),
            assets,
            liabilities,
            percentage_leverages: Vec::new(),
        }
    }
}

fn main() {
    let mut companies = Vec::new();

    // Input username and password
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let username = prompt_user_input();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = prompt_user_input();

    
    if validate_username(&username) && validate_password(&password) {
        // Input company data
        let company_data = [
            ("Cadbury Nigeria Plc", "1965", 15000000.0, 5500000.0),
            ("Champion Breweries Plc", "1974", 25000000.0, 8000000.0),
            ("Dangote Sugar Rfinery Plc", "1970", 18000000.0, 10000000.0),
            ("Flour Mills Nigeria Plc", "1960", 32000000.0, 4000000.0),
            ("Nestle Nigeria Plc", "1961", 8000000.0, 1500000.0),
            ("Unilever Nigeria Plc", "1923", 37000000.0, 11000000.0 ),
            ("Honeywell Nigeria Plc", "1906", 34000000.0, 9000000.0 ),
            ("Nigerian Breweries Plc", "1946", 30000000.0, 12000000.0),
        ];

        for &(name, founded, shares, liabilities) in &company_data {
            println!("Enter data for {}", name);

            let mut company = Company::new(name, founded, shares, liabilities);

            if shares > 20_000_000.0 {
                input_leverages(&mut company);
                save_leverages_to_file(&company);
            }

            if liabilities < 10_000_000.0 {
                calculate_5_percent(&mut company);
            }

            companies.push(company);
        }

        
        save_companies_to_file(&companies);
    } else {
        println!("Invalid username or password. Exiting.");
    }
}

fn validate_username(username: &str) -> bool {
    
    true
}

fn validate_password(password: &str) -> bool {
   
    true
}

fn prompt_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn input_leverages(company: &mut Company) {
    println!("Enter the number of percentage leverages used by the company:");
    let number_of_leverages: f32 = prompt_user_input().parse().unwrap();

    for _ in 0..number_of_leverages {
        println!("Enter the multiple of percentage leverages used by the company:");
        let leverage: f32 = prompt_user_input().parse().unwrap();
        company.percentage_leverages.push(leverage);
    }
}

fn save_companies_to_file(companies: &[Company]) {
    let mut file = File::create("breweries.txt").unwrap();

    for company in companies {
        writeln!(file, "{}|{}|{}|{}|{:?}", company.name, company.founded, company.assets, company.liabilities, company.percentage_leverages).unwrap();
    }
}

fn save_leverages_to_file(company: &Company) {
    let mut file = File::create(format!("{}_leverages.txt", company.name)).unwrap();

    for &leverage in &company.percentage_leverages {
        writeln!(file, "{}", leverage).unwrap();
    }
}

fn calculate_5_percent(company: &mut Company) {
    for leverage in &mut company.percentage_leverages {
        *leverage = (*leverage * 5) / 100;
    }
}
