use std::io;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

fn get_input(filename: String) -> Vec<i32> {
    let file = File::open(filename).expect("File could not be read.");

    let mut line = String::new();
    BufReader::new(file).read_line(&mut line).expect("Read error.");

    line.split(",").map(|x|x.to_string().parse().unwrap()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usages: ./main program.txt");
        return;
    }

    let mut imap: HashMap<i32,i32> = HashMap::new();

    let mut truth_table: Vec<Vec<i32>> = Vec::new();

    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        if  a != b && a != c && a != d && a != e
                            && b != c && b != d && b != e
                            && c != d && c != e
                            && d != e
                            {
                                truth_table.push([a,b,c,d,e].to_vec());
                            }
                    }
                }
            }
        }
    }


    let mut max: i32 = 0;
    // truth_table = [
        // [4,3,2,1,0].to_vec(),
        // [0,1,2,3,4].to_vec(),
        // [1,0,4,3,2].to_vec(),
    // ].to_vec();

    for input in truth_table {
        let mut result = 0;

        println!("a {}, b {}, c {}, d {}, e {}", input[0], input[1], input[2], input[3], input[4]);
        let mut last_out = 0;
        for level in input {
            imap.remove(&0);
            imap.remove(&1);
            imap.insert(1, last_out);
            imap.insert(0, level);
            println!("map {:#?}", imap);
            last_out = computer(&mut get_input(args[1].to_string()), &imap, true);
            println!("last out {}", last_out  );
            result = last_out;
        }

        if max < result {
            max = result;
        }
    }

    println!("MAX {}", max);
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
fn computer(intcode: &mut Vec<i32>, inputs: &HashMap<i32,i32>, return_on_out: bool) -> i32 {

    let mut op_ptr: usize = 0;

    let code_size: usize = intcode.len();
    let mut inputCount = 0;

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
            // ADD and MULTIPLY
            1 | 2 => {
                let ref1 = intcode[p1];
                let ref2 = intcode[p2];
                let ref3 = intcode[p3];
                let val1 = if p1m == 0 { intcode[ref1 as usize] } else { ref1 };
                let val2 = if p2m == 0 { intcode[ref2 as usize] } else { ref2 };
                let index = ref3;

                let answer;

                if op_code == 1 {
                    // ADD
                    answer = val1 + val2;
                } else {
                    // MULTIPLY
                    answer = val1 * val2;
                }

                intcode[index as usize] = answer;
            },
            // INPUT
            3 => {
                op_len = 2;

                let ref1 = intcode[p1];

                let mut num: i32 = -1;

                // Input passed in or...
                match inputs.get(&inputCount) {
                    // Get from input map.
                    Some(item) => num = *item,
                    // Get from STDIN.
                    None => {
                        let mut value = String::new();
                        println!("Please enter instruction: {}", inputCount);
                        io::stdin().read_line(&mut value).expect("Failed to read line");
                        num = value.trim().parse().expect("Please type a number!");
                    },
                }

                intcode[ref1 as usize] = num;

                inputCount = inputCount + 1;
            },
            // OUTPUT
            4 => {
                op_len = 2;

                let ref1 = intcode[p1];
                let val1 = if p1m == 0 { intcode[ref1 as usize] } else { ref1 };

                if return_on_out {
                    return val1;
                }

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

    return 0;
}