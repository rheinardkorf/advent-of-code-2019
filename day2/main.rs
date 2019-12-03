fn main() {

    let intcode: Vec<u32> = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0].to_vec();
    let mut alarm_state: Vec<u32> = [1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0].to_vec();

    println!("[PART 1]:: {}", computer(&mut alarm_state)[0]);

    let desired_result: u32 = 19690720;
    println!("[PART 2]:: Desired {}, Answer: {}", desired_result, compound_noun_verb(intcode, desired_result));
}

fn compound_noun_verb(intcode: Vec<u32>, desired_result: u32) -> u32 {

    let upper_limit = intcode.len();

    for noun in 0..upper_limit {
        for verb in 0..upper_limit {
            let mut test: Vec<u32> = intcode.to_vec();
            test[1] = noun as u32;
            test[2] = verb as u32;

            let result = computer(&mut test);
            if desired_result == result[0] {
                return (100 * noun + verb) as u32;
            }
        }
    }

    return 0;
}

fn op_len(code: u32) -> usize {
    if code == 99 { return 1; }
    return 4;
}

fn op_exec(intcode: &Vec<u32>, code: u32, p1: usize, p2: usize) -> u32 {
    let ref1: usize = intcode[p1] as usize;
    let ref2: usize = intcode[p2] as usize;
    let val1 = intcode[ref1];
    let val2 = intcode[ref2];

    match code {
        1 => {
            return val1 + val2;
        },
        2 => {
            return val1 * val2;
        },
        _ => panic!("Unknown op code: {}", code),
    }
}

fn computer(intcode: &mut Vec<u32>) -> &Vec<u32> {

    let mut op_ptr: usize = 0;

    let code_size: usize = intcode.len();

    loop {
        let op_code = intcode[op_ptr];

        if op_ptr >= code_size || op_code == 99 {
            break;
        }

        let calculated: u32 = op_exec(&intcode, op_code, op_ptr + 1, op_ptr + 2);
        let dst = intcode[op_ptr+3] as usize;

        intcode[dst] = calculated;

        op_ptr = op_ptr + op_len(op_code)
    }

    return intcode;
}