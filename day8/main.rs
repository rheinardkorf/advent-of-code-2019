use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type LayerArray = Vec<Vec<u32>>;
type Layer = Vec<u32>;

fn get_input(filename: String) -> Vec<u32> {
    let file = File::open(filename).expect("File could not be read.");

    let mut line = String::new();
    BufReader::new(file).read_line(&mut line).expect("Read error.");

    line.chars().map(|x|x.to_digit(10).unwrap()).collect::<Vec<u32>>()
}

fn meta(layer: Layer) -> (u32,u32,u32) {

    let mut zero = 0;
    let mut one = 0;
    let mut two = 0;

    for i in layer {
        zero = if i == 0 { zero + 1 } else { zero };
        one = if i == 1 { one + 1 } else { one };
        two = if i == 2 { two + 1 } else { two };
    }

    return (zero, one, two);
}

fn render(l: Layer, w: usize) {

    let mut x = 0;
    for i in l {
        x = x + 1;

        if i == 1 {
            print!("*");
        } else {
            print!(" ")
        }

        if x == w {
            println!("");
            x = 0;
        }

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usages: ./main program.txt");
        return;
    }

    let encoded = get_input(args[1].to_string());

    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let layer_count = encoded.len() / layer_size;

    let mut layers: LayerArray = Vec::new();

    for i in 0..layer_count {
        let cursor = layer_size * i;
        let mut layer: Layer = Vec::new();

        // Current layer is (layer_size * i). Current cell is j.
        for j in 0..layer_size {
            layer.push(encoded[j + cursor]);
        }

        layers.push(layer.to_vec());
    }

    let mut check_layer: Layer = Vec::new();

    for layer in layers.to_vec() {
        if check_layer.len() == 0 {
            check_layer = layer.to_vec();
            continue;
        }

        let check_meta = meta(check_layer.to_vec());
        let layer_meta = meta(layer.to_vec());

        if layer_meta.0 < check_meta.0 {
            check_layer = layer.to_vec();
        }
    }

    let layer_meta = meta(check_layer.to_vec());

    println!("Ones * Twos = {}", layer_meta.1 * layer_meta.2);

    let mut final_layer: Layer = Vec::new();

    for i in 0..layer_size {
        for n in 0..layer_count {
            if layers[n][i] != 2 {
                final_layer.push(layers[n][i]);
                break;
            }
        }
    }

    render(final_layer.to_vec(), width);
}