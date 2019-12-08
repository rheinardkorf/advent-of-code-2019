use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;


type Orbit = Vec<String>;
type OrbitArray = Vec<Orbit>;


fn get_orbits(filename: String) -> OrbitArray {
    let file = File::open(filename).expect("File could not be read.");

    let orbits: OrbitArray = BufReader::new(file)
        .lines()
        .map(|line|line.unwrap().split(")").map(|x|x.to_string()).collect::<Orbit>())
        .collect();

    orbits
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usages: ./main diagnostic.txt");
        return;
    }

    let orbits = get_orbits(args[1].to_string());

    let mut tree:BTreeMap<String,String> = BTreeMap::new();
    for orbit in orbits {
        tree.insert(orbit[1].clone(), orbit[0].clone());
    }

    // Part 1
    println!("Part 1: {:#?}", counter(&tree));

    // Part 2
    println!("Part 2: {}", transfers(&tree, "YOU".to_string(), "SAN".to_string()) );
}

fn transfers(tree: &BTreeMap<String,String>, object_a: String, object_b: String) -> i32 {

    let mut item: String;
    let mut a_and_b: OrbitArray = Vec::new();

    // Get the paths.
    for value in [object_a,object_b].to_vec() {
        item = value.to_string();
        let mut temp: Orbit = Vec::new();
        loop {
            match tree.get(&item) {
                Some(leaf) => {
                    item = leaf.to_string();
                    temp.push(leaf.to_string());
                },
                None => { break; }
            }
        }
        a_and_b.push(temp);
    }

    // Get diff.
    let mut unique: Orbit = Vec::new();

    for point in a_and_b[0].to_vec() {
        if !a_and_b[1].contains(&point.to_string()) {
            unique.push(point.to_string());
        }
    }
    for point in a_and_b[1].to_vec() {
        if !a_and_b[0].contains(&point.to_string()) {
            unique.push(point.to_string());
        }
    }

    return unique.len() as i32;
}

fn counter(tree: &BTreeMap<String,String>) -> i32 {

    let mut total: i32 = 0;
    let mut item: String;
    for value in tree.values() {
        item = value.to_string();
        loop {
            match tree.get(&item) {
                Some(leaf) => {
                    total = total + 1;
                    item = leaf.to_string();
                },
                None => { break; }
            }
        }
        total = total + 1;
    }

    total
}