use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let mut count: f32 = 0.0;

    let file = File::open("file.txt")?;
    for line in BufReader::new(file).lines() {
        let value: f32 = line?.parse().unwrap();

        // part 1
        // count = count + ((value/3.0).floor() - 2.0);

        count = count + module_consumption(value, 0.0);
    }
    println!("{}", count);
    Ok(())
}

// part 2
fn module_consumption(fuel: f32, count: f32) -> f32 {
    if fuel < 0.0 {
        return count - fuel;
    }

    if fuel == 0.0 {
        return count;
    }

    let calculated_fuel = (fuel/3.0).floor() - 2.0;
    let sum = count + calculated_fuel;

    return module_consumption(calculated_fuel, sum);
}