fn main() {

    let mut valids: Vec<u32> = Vec::new();

    for i in 138241..674034 {
        if is_valid(i) {
            valids.push(i);
        }
    }

    println!("Valid count: {}", valids.len());

}

fn is_valid(pass: u32) -> bool {

    let mut has_adj = false;
    let mut has_2 = false;
    let mut adj_count = 1;
    let mut prev = 0;
    let mut res = pass;

    for i in (0..6).rev() {
        let dec = 10_u32.pow(i);
        let dig = res / dec;
        res = res - (dig*dec);

        if dig < prev { return false; }
        if dig == prev {
            has_adj = true;

            // Part 2.
            adj_count = adj_count + 1;
        } else {
            // Part 2.
            if adj_count == 2 {
                has_2 = true;
            }
            adj_count = 1;
        }

        prev = dig;
    }

    // Part 2.
    if adj_count == 2 {
        has_2 = true;
    }

    // Part 1.
    // return has_adj;

    // Part 2.
    return has_adj && has_2;
}