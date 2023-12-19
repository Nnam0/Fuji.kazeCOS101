struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn calc_total(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let laptops: Vec<Laptop> = vec![
        Laptop { brand: String::from("HP"), price: 650_000 },
        Laptop { brand: String::from("IBM"), price: 755_000 },
        Laptop { brand: String::from("Toshiba"), price: 550_000 },
        Laptop { brand: String::from("HP"), price: 850_000 },
    ];

    // Example usage of calc_total
    for laptop in &laptops {
        let total_cost = laptop.calc_total(3);
        println!("Total cost of {} laptops: {}", laptop.brand, total_cost);
    }
}
