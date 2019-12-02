use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let mut count: i32 = 0;

    let file = File::open("file.txt")?;
    for line in BufReader::new(file).lines() {
        let value: i32 = line?.parse().unwrap();
        count = count + module_consumption(value, 0);
    }
    println!("{}", count);
    Ok(())
}

fn module_consumption(fuel: i32, count: i32) -> i32 {
    if fuel < 0 {
        return count - fuel;
    }

    let calculated_fuel = fuel / 3 - 2;
    let sum = count + calculated_fuel;

    return module_consumption(calculated_fuel, sum);
}