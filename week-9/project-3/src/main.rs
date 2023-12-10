use std::fs::File;
use std::io::Write;

fn main() {

    let name_of_commisioner = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbonna", "Adewale Jimoh Akonbi", "Osazuwa Faith Etieye"];
    let minister = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geo_zone = vec!["South West" ,"North East", "South South", "South West", "South East"];
    

    let file_name ="convicted_minister.txt";

    let mut file = File::create(file_name).expect("Error: Unable to create or open the file.");

    file.write_all(b"           CONVICTED MINISTERS\n").expect("Error writing to file");

    file.write_all(format!("{:<50} {:<50} {:<30}\n", "Name", "Minister", "Geo_zone")
        .as_bytes())
        .expect("Error writing to file");

    for n in 0..name_of_commisioner.len() {
        file.write_all(
            format!("{:<50} {:<50} {:<30}\n", name_of_commisioner[n], minister[n], geo_zone[n])
            .as_bytes()
        )
        .expect("Error writing to file");
    }

    println!("Ministers details have been written to {}", file_name);
}