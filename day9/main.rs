use std::io;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

mod intcode;

fn get_program(filename: String) -> Vec<i64> {
    let file = File::open(filename).expect("File could not be read.");

    let mut line = String::new();
    BufReader::new(file).read_line(&mut line).expect("Read error.");

    line.split(",").map(|x|x.to_string().parse().unwrap()).collect()
}


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usages: ./main diagnostic.txt");
        return;
    }

    let program = get_program(args[1].to_string());
    let mut io_bus: Vec<i64> = Vec::new();

    let result = intcode::computer(&program.to_vec(), &mut io_bus.to_vec(), true, false);
    println!("{:?}", result.1);
}