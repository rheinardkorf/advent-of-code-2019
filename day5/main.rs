use std::io;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};


fn get_input(filename: String) -> Vec<i32> {
    let file = File::open(filename).expect("File could not be read.");

    let mut line = String::new();
    BufReader::new(file).read_line(&mut line).expect("Read error.");

    line.split(",").map(|x|x.to_string().parse().unwrap()).collect()
}

// Examples:
//
// 1101,100,-1,4,0  --->  99
// 1101,100,-1,4,99
//
//


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usages: ./main filename");
        return;
    }

    computer(&mut get_input(args[1].to_string()));
}

fn decode_instruction(i: i32) -> (i32,i32,i32,i32) {
    (
        i % 100000 / 10000,
        i % 10000 / 1000,
        i % 1000 / 100,
        i % 100
    )
}

// ABCDE
//  1002
// DE - two-digit opcode,      02 == opcode 2
//  C - mode of 1st parameter,  0 == position mode
//  B - mode of 2nd parameter,  1 == immediate mode
//  A - mode of 3rd parameter,  0 == position mode,
//                                   omitted due to being a leading zero
fn computer(intcode: &mut Vec<i32>) {

    let mut op_ptr: usize = 0;

    let code_size: usize = intcode.len();

    loop {

        let mut op_len: usize = 4;
        let mut jump: bool = false;
        let p1 = op_ptr + 1;
        let p2 = op_ptr + 2;
        let p3 = op_ptr + 3;

        let ( p3m, p2m, p1m, op_code ) = decode_instruction(intcode[op_ptr]);

        if op_ptr >= code_size || op_code == 99 {
            break;
        }

        match op_code {
            // ADD
            1 => {
                let ref1 = intcode[p1];
                let ref2 = intcode[p2];
                let ref3 = intcode[p3];
                let val1 = if p1m == 0 { intcode[ref1 as usize] } else { ref1 };
                let val2 = if p2m == 0 { intcode[ref2 as usize] } else { ref2 };
                let index = ref3;
                let answer = val1 + val2;
                intcode[index as usize] = answer;
            },
            // MULTIPLY
            2 => {
                let ref1 = intcode[p1];
                let ref2 = intcode[p2];
                let ref3 = intcode[p3];
                let val1 = if p1m == 0 { intcode[ref1 as usize] } else { ref1 };
                let val2 = if p2m == 0 { intcode[ref2 as usize] } else { ref2 };
                let index = ref3;
                let answer = val1 * val2;
                intcode[index as usize] = answer;
            },
            // INPUT
            3 => {
                op_len = 2;

                let ref1 = intcode[p1];

                // Get from input
                let mut value = String::new();
                println!("Please enter instruction:");
                io::stdin().read_line(&mut value).expect("Failed to read line");

                let num: i32 = value.trim().parse().expect("Please type a number!");

                intcode[ref1 as usize] = num;
            },
            // OUTPUT
            4 => {
                op_len = 2;

                let ref1 = intcode[p1];
                let val1 = if p1m == 0 { intcode[ref1 as usize] } else { ref1 };
                println!("OUTPUT => {}", val1);
            },
            // JUMP IF TRUE
            5 => {
                op_len = 3;

                let ref1 = intcode[p1];
                let ref2 = intcode[p2];
                let val1 = if p1m == 0 { intcode[ref1 as usize] } else { ref1 };
                let val2 = if p2m == 0 { intcode[ref2 as usize] } else { ref2 };

                if val1 != 0 {
                    jump = true;
                    op_ptr = val2 as usize;
                }
            },
            // JUMP IF FALSE
            6 => {
                op_len = 3;

                let ref1 = intcode[p1];
                let ref2 = intcode[p2];
                let val1 = if p1m == 0 { intcode[ref1 as usize] } else { ref1 };
                let val2 = if p2m == 0 { intcode[ref2 as usize] } else { ref2 };

                if val1 == 0 {
                    jump = true;
                    op_ptr = val2 as usize;
                }
            },
            // LESS THAN
            7 => {
                op_len = 4;

                let ref1 = intcode[p1];
                let ref2 = intcode[p2];
                let ref3 = intcode[p3];
                let val1 = if p1m == 0 { intcode[ref1 as usize] } else { ref1 };
                let val2 = if p2m == 0 { intcode[ref2 as usize] } else { ref2 };
                let index = ref3 as usize;

                if val1 < val2 {
                    intcode[index] = 1;
                } else {
                    intcode[index] = 0;
                }
            },
            // EQUAL
            8 => {
                op_len = 4;

                let ref1 = intcode[p1];
                let ref2 = intcode[p2];
                let ref3 = intcode[p3];
                let val1 = if p1m == 0 { intcode[ref1 as usize] } else { ref1 };
                let val2 = if p2m == 0 { intcode[ref2 as usize] } else { ref2 };
                let index = ref3 as usize;

                if val1 == val2 {
                    intcode[index] = 1;
                } else {
                    intcode[index] = 0;
                }
            },
            99 => {
                break;
            }
            _ => {
                println!("{:#?}", intcode);
                panic!("Unknown op code: {}", op_code)
            },
        }

        if !jump {
            op_ptr = op_ptr + op_len;
        }
    }
}