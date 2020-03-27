use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").unwrap();
    let mut total_fuel = 0.0_f64;
    let mut total_fuel_p1 = 0.0_f64;
    for x in contents.split("\n") {
        let module_weight: f64 = x.parse().unwrap();
        let module_fuel = get_fuel_weight(module_weight);
        total_fuel_p1 +=  module_fuel;
        let mut fuel_for_fuel: f64 = 0.0_f64;
        let mut tmp: f64 = module_fuel;
        while tmp > 0.0 {
            tmp = get_fuel_weight(tmp);
            if tmp <= 0.0 { tmp = 0.0; }
            fuel_for_fuel += tmp;
        }
        total_fuel += module_fuel + fuel_for_fuel;
    }
    println!("The first part answer: {}", total_fuel_p1);
    println!("The second part answer: {}", total_fuel);
}

fn get_fuel_weight(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}