use std::io;

// ABCDE
//  1002
// DE - two-digit opcode,      02 == opcode 2
//  C - mode of 1st parameter,  0 == position mode
//  B - mode of 2nd parameter,  1 == immediate mode
//  A - mode of 3rd parameter,  0 == position mode,
//                                   omitted due to being a leading zero
pub fn computer(program: &Vec<i64>, io_bus: &mut Vec<i64>, echo_out: bool, feedback: bool) -> (Vec<i64>, Vec<i64>) {

    let additional_memory = program.len() * 10;

    let mut memory = Vec::with_capacity(additional_memory as usize);

    println!("Initialize computer memory...");
    for i in 0..additional_memory {
        memory.push(0);
    }

    let mut intcode = program.to_vec();
    intcode.append(&mut memory);

    let mut op_ptr: usize = 0;

    let code_size: usize = intcode.len();
    let mut relative_base = 0;

    loop {
        let mut op_len: usize = 4;
        let mut jump: bool = false;
        let p1 = op_ptr + 1;
        let p2 = op_ptr + 2;
        let p3 = op_ptr + 3;

        let (p3m, p2m, p1m, op_code) = decode_instruction(intcode[op_ptr]);

        if op_ptr >= code_size || op_code == 99 {
            return (intcode.to_vec(), io_bus.to_vec());
        }

        match op_code {
            // ADD and MULTIPLY
            1 | 2 => {
                op_len = 4;

                let val1 = get_by_mode(&intcode.to_vec(), p1m, p1 as i64, relative_base);
                let val2 = get_by_mode(&intcode.to_vec(), p2m, p2 as i64, relative_base);
                let index = get_index(&intcode.to_vec(), p3m, p3 as i64, relative_base);
                let answer;

                if op_code == 1 {
                    // ADD
                    answer = val1 + val2;
                } else {
                    // MULTIPLY
                    println!("in {} in {}", val1, val2);
                    answer = val1 * val2;
                }

                intcode[index as usize] = answer;
            }
            // INPUT
            3 => {
                op_len = 2;

                let mut num: i64 = 0;
                let index = get_index(&intcode.to_vec(), p1m, p1 as i64, relative_base);

                if io_bus.len() != 0 {
                    if let Some((first, elements)) = io_bus.split_first() {
                        num = *first;
                        *io_bus = elements.to_vec();
                    }
                } else {
                    // Get from input
                    let mut value = String::new();
                    println!("Please enter instruction:");
                    io::stdin()
                        .read_line(&mut value)
                        .expect("Failed to read line");

                    num = value.trim().parse().expect("Please type a number!");
                }

                intcode[index] = num;
            }
            // OUTPUT
            4 => {
                op_len = 2;
                let val1 = get_by_mode(&intcode.to_vec(), p1m, p1 as i64, relative_base);
                io_bus.push(val1);
                if echo_out {
                    println!("> {}", val1);
                }
            }
            // JUMP IF TRUE
            5 => {
                op_len = 3;

                let val1 = get_by_mode(&intcode.to_vec(), p1m, p1 as i64, relative_base);
                let val2 = get_by_mode(&intcode.to_vec(), p2m, p2 as i64, relative_base);

                if val1 != 0 {
                    jump = true;
                    op_ptr = val2 as usize;
                }
            }
            // JUMP IF FALSE
            6 => {
                op_len = 3;

                let val1 = get_by_mode(&intcode.to_vec(), p1m, p1 as i64, relative_base);
                let val2 = get_by_mode(&intcode.to_vec(), p2m, p2 as i64, relative_base);

                if val1 == 0 {
                    jump = true;
                    op_ptr = val2 as usize;
                }
            }
            // LESS THAN
            7 => {
                op_len = 4;

                let val1 = get_by_mode(&intcode.to_vec(), p1m, p1 as i64, relative_base);
                let val2 = get_by_mode(&intcode.to_vec(), p2m, p2 as i64, relative_base);
                let index = get_index(&intcode.to_vec(), p3m, p3 as i64, relative_base);

                if val1 < val2 {
                    intcode[index] = 1;
                } else {
                    intcode[index] = 0;
                }
            }
            // EQUAL
            8 => {
                op_len = 4;

                let val1 = get_by_mode(&intcode.to_vec(), p1m, p1 as i64, relative_base);
                let val2 = get_by_mode(&intcode.to_vec(), p2m, p2 as i64, relative_base);
                let index = get_index(&intcode.to_vec(), p3m, p3 as i64, relative_base);

                if val1 == val2 {
                    intcode[index] = 1;
                } else {
                    intcode[index] = 0;
                }
            }
            // RELATIVE BASE ADJUST
            9 => {
                op_len = 2;

                let val1 = get_by_mode(&intcode.to_vec(), p1m, p1 as i64, relative_base);
                relative_base = relative_base + val1;
            }
            99 => {
                return (intcode.to_vec(), io_bus.to_vec());
            }
            _ => {
                println!("{:#?}", intcode);
                panic!("Unknown op code: {}", op_code)
            }
        }

        if !jump {
            op_ptr = op_ptr + op_len;
        }

        // println!("{:?}", io_bus);
        // return (Vec::new(),Vec::new());
    }
}

fn decode_instruction(i: i64) -> (i64, i64, i64, i64) {
    (
        i % 100000 / 10000,
        i % 10000 / 1000,
        i % 1000 / 100,
        i % 100,
    )
}

fn get_by_mode(program: &Vec<i64>, mode: i64, pointer: i64, relative_base: i64) -> i64 {
    // println!("mode {}, pointer {}, ref {}", mode, pointer, program[pointer as usize]);
    let parameter = program[pointer as usize];

    match mode {
        1 => return parameter,
        2 => return program[(parameter + relative_base) as usize],
        _ => return program[parameter as usize],
    }
}

fn get_index(program: &Vec<i64>, mode: i64, pointer: i64, relative_base: i64) -> usize {
    let parameter = program[pointer as usize];

    match mode {
        1 => panic!("Please don't use mode 1 for param 3"),
        2 => return (relative_base + parameter) as usize,
        _ => return parameter as usize,
    }
}