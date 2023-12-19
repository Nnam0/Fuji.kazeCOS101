// Define the Rectangle structure
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation of Rectangle methods
impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        // Use the . operator to fetch the value of a field via the self keyword
        self.width * self.height
    }
}

fn main() {
    // Instantiate the structure
    let small = Rectangle {
        width: 10,
        height: 20,
    };

    // Print the rectangle's area
    println!(
        "Width is {}\nHeight is {}\nArea of Rectangle is {}",
        small.width,
        small.height,
        small.area()
    );
}
