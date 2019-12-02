fn main() {

    let mut intcode: Vec<u64> = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0].to_vec();
    let mut alarm_state: Vec<u64> = [1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0].to_vec();

    let example: Vec<u64> = [2,3,0,3,99].to_vec();

    // println!("Normal:\n {:?}", computer(intcode));
    println!("Alarm:\n {:?}", computer(alarm_state));
    // println!("Example:\n {:?}", computer(example));

}

fn computer(mut intcode: Vec<u64>) -> Vec<u64> {

    let num_ops: usize = intcode.len() / 4;

    for i in 0..num_ops {

        let index = i*4;

        let item = intcode[index];

        if item == 99 {
            break;
        }

        let val1_position: usize = intcode[index+1] as usize;
        let val2_position: usize = intcode[index+2] as usize;
        let storage_position: usize = intcode[index+3] as usize;

        let mut calculated: u64 = 0;
        let val1 = intcode[val1_position];
        let val2 = intcode[val2_position];

        if item == 1 {
            calculated = val1 + val2;
            intcode[storage_position] = calculated;
        }

        if item == 2 {
            println!("{}, {}", val1, val2);
            calculated = val1 * val2;
            intcode[storage_position] = calculated;
        }

    }
    return intcode;
}