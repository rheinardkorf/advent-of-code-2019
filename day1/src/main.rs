use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usages: ./main filename");
        return Ok(());
    }

    let file = File::open(&args[1])?;

    let modules: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|x|line_to_int(x))
        .collect();

    let results = modules.iter().flat_map(|x| process(*x));
    let output = results.fold(0, |acc, x| acc + x);

    println!("{}", output);
    Ok(())
}

fn line_to_int(x: Result<String>) -> i32 {
    let n: String = x.unwrap();
    let fuel: i32 = n.parse().unwrap();
    return fuel;
}

fn process(x: i32) -> Vec<i32> {
    let mut n: i32 = x;
    let mut result: Vec<i32> = Vec::new();
    loop {
        let new_n = n / 3 - 2;
        if new_n < 0 { break; };
        n = new_n;
        result.push(new_n);
    }
    result
}