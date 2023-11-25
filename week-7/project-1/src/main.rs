use std::io;

fn get_aot(height:f64,base1:f64,base2:f64)->f64{
        let a:f64 = height/2.0*(base1 + base2);
        return a;
}

fn get_aor(diagonal1:f64,diagonal2:f64)->f64{
    let b:f64 = 0.5 * diagonal1 * diagonal2;
    return b;
}

fn get_aop(base_1:f64,altitude:f64)->f64{
    let c:f64 = base_1 * altitude;
    return c;
}

fn get_aoc(length:f64)->f64{
    let d:f64 = 6.0 * (length).powf(2.0);
    return d;
}

fn get_voc(radius:f64,height_c:f64)->f64{
    let e:f64 = 3.14 * (radius).powf(2.0) * height_c;
    return e;
}

fn main(){
    let mut base1 = String::new();
    let mut base2 = String::new();
    let mut height = String::new();

    let mut diagonal1 = String::new();
    let mut diagonal2 = String::new();

    let mut base_1 = String::new();
    let mut altitude = String::new();

    let mut length = String::new();

    let mut radius = String::new();
    let mut height_c = String::new();

    let mut input = String::new();
    println!("which formula do you want to use");
    println!("formula for area of trapezuim(input aot)");
    println!("formula for area of rhombus(input aor)");
    println!("formula for area of parallelogram(input aop)");
    println!("formula for area of cube(input aoc)");
    println!("formula for volume of cylinder(input voc)");
    io::stdin().read_line(&mut input).expect("we do not have that formula input yet");
    let select_formula = input.trim();

    if select_formula == "aot"{
        println!("input base1");
        io::stdin().read_line(&mut base1).expect("we do not have that formula input yet");
        let base1:f64 = base1.trim().parse().expect("invalid");

        println!("input base2");
        io::stdin().read_line(&mut base2).expect("we do not have that formula input yet");
        let base2:f64 = base2.trim().parse().expect("invalid");

        println!("input height");
        io::stdin().read_line(&mut height).expect("we do not have that formula input yet");
        let height:f64 = height.trim().parse().expect("invalid");

        println!("area of trapezium is {}",get_aot(height, base1, base2));
    }else if select_formula == "aor" {
        println!("input diagonal 1");
        io::stdin().read_line(&mut diagonal1).expect("we do not have that formula input yet");
        let diagonal1:f64 = diagonal1.trim().parse().expect("invalid");

        println!("input diagonal 2");
        io::stdin().read_line(&mut diagonal2).expect("we do not have that formula input yet");
        let diagonal2:f64 = diagonal2.trim().parse().expect("invalid");

        println!("area of rhombus is: {}",get_aor(diagonal1, diagonal2));
    }else if select_formula == "aop" {
        println!("input base_1");
        io::stdin().read_line(&mut base_1).expect("we do not have that formula input yet");
        let base_1:f64 = base_1.trim().parse().expect("invalid");

        println!("input altitude");
        io::stdin().read_line(&mut altitude).expect("we do not have that formula input yet");
        let altitude:f64 = altitude.trim().parse().expect("invalid");

        println!("area of parallelogram is: {}",get_aop(base_1, altitude));
    }else if select_formula == "aoc" {
        println!("input length of any side of cube");
        io::stdin().read_line(&mut length).expect("we do not have that formula input yet");
        let length:f64 = length.trim().parse().expect("invalid");

        println!("area of cube is : {}",get_aoc(length));
    }else if select_formula == "voc" {
        println!("input radius");
        io::stdin().read_line(&mut radius).expect("we do not have that formula input yet");
        let radius:f64 = radius.trim().parse().expect("invalid");

        println!("input height of cylinder");
        io::stdin().read_line(&mut height_c).expect("we do not have that formula input yet");
        let height_c:f64 = height_c.trim().parse().expect("invalid");

        println!("volume of cylinder is : {}",get_voc(radius, height_c));
    }else {
        println!("there is no formula for that yet");
    }

}

