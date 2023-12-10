use std::io::Write;

fn main() {
    let lager = vec![
        "33 export\n".to_string(),
        "Desperados\n".to_string(),
        "Goldberg\n".to_string(),
        "Gulder\n".to_string(),
        "Heineken\n".to_string(),
        "Star\n\n".to_string(),
    ];
    let stout = vec![
        "Legend\n".to_string(),
        "Turbo king\n".to_string(),
        "Williams\n\n".to_string(),
    ];
    let non_alcoholic = vec![
        "Maltina\n".to_string(),
        "Amstel malt\n".to_string(),
        "Malta Gold\n".to_string(),
        "Fayrouz\n\n".to_string(),
    ];

    let mut file = std::fs::File::create("Drink Categories.txt").expect("create failed");

    file.write_all("Welcome to Nigerian Breweries plc,\nThese are our High-Quality Categories of Drinks \n\n".as_bytes())
        .expect("write failed");

    write_category_to_file(&mut file, "Under Lager drinks", &lager);
    write_category_to_file(&mut file, "Under Stout drinks", &stout);
    write_category_to_file(&mut file, "Under non-alcoholic drinks", &non_alcoholic);

    println!("\nAll done.");
}

fn write_category_to_file(file: &mut std::fs::File, category_title: &str, drinks: &[String]) {
    file.write_all(format!("{}:\n\n", category_title).as_bytes())
        .expect("Failed to write into file");

    for drink in drinks {
        file.write_all(drink.as_bytes()).expect("Failed to write into file");
    }
}