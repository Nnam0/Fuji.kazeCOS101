use std::fs::File;
use std::io::Write;

struct Company {
    name: String,
    year_founded: u32,
    share: f64,
    liabilities: f64,
}

impl Company {
    fn leverage_percentage(&self) -> f64 {
        (self.share - self.liabilities) / self.share * 100.0
    }
}

fn main() {
    let username = get_valid_username();
    let password = get_valid_password();

    if authenticate_user(&username, &password){

        let companies: Vec<Company> = vec![
        Company { name: String::from("Cadbury"), year_founded: 1965, company_share: 15_000_000.0, company_liabilities:5_500_000.0 },
        Company { name: String::from("Champion"), year_founded: 1974,  company_share: 25_000_000.0, company_liabilities:8_000_000.0 },
        Company { name: String::from("Dangote"), year_founded: 1970, company_share: 18_000_000.0, company_liabilities:10_000_000.0 },
        Company { name: String::from("Flour Mills"), year_founded: 1960,  company_share: 32_000_000.0, company_liabilities:4_000_000.0 },
        Company { name: String::from("Nestle"), year_founded: 1961,  company_share: 8_000_000.0, company_liabilities:1_500_000.0 },
        Company { name: String::from("Unilever"), year_founded: 1923,  company_share: 37_000_000.0, company_liabilities:11_000_000.0 },
        Company { name: String::from("Honeywell"), year_founded: 1906,  company_share: 34_000_000.0, company_liabilities:9_000_000.0 },
        Company { name: String::from("Nigerian_Br"), year_founded: 1946,  company_share: 30_000_000.0, company_liabilities:12_000_000.0 },
        ];

        let mut file = File::create("breweries.txt").expect("Unable to create a file");

        for company in companies.iter(){
            let leverage_percentage = company.leverage_percentage();

            writeln!(&mut file,  "Company: {}, Founded: {}, Share: {}, Liabilities: {}, Leverage Percentage: {}%",
                company.name, company.year_founded, company.share, company.liabilities, leverage_percentage).expect("Unable to write to file");

            if company.share > 20_000_000{
                let multiple = leverage_percentage * 2.0;
                writeln!(&mut file, "Multiple of leverage: {}%", multiple).expect("Unable to write to file");
            }

            if companyliabilities < 10_000_000 {
                let five_percent = leverage_percentage * 0.05;
                writeln!(&mut file, "5% of leverage: {}%", five_percent).expect("Unable to write to file");
            }
        }
        println!("Data saved to breweries.txt");
    } else {
        println!("Authentication failed");
    }

}

fn get_valid_username() -> String {
    println!("Enter username");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");
    username.trim().to_string()
}

fn get_valid_password() -> String {
    println!("Enter password");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    password.trim().to_string()
}

fn authenticate_user(username: &str, password: &str) -> bool{
    username == "admin" && password == "password" 
}