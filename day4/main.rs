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

    // Loop through each digit going from left to right.
    for i in (0..6).rev() {

        // This will create the following upon each loop:
        // 100k 10k 1k 100 10 1
        // These are used to isolate the most left digit for processing.
        let dec = 10_u32.pow(i);

        // Get the digit.
        let dig = res / dec;

        // Remove the digit and work out what remains.
        res = res - (dig*dec);

        // Can be the same, but should not get smaller.
        if dig < prev { return false; }

        // Test for adjacency.
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

        // Remember the last digit processed.
        prev = dig;
    }

    // Part 2.
    // If we reach the end of the loop we never get to test this.
    // So lets do it now!
    if adj_count == 2 {
        has_2 = true;
    }

    // Part 1.
    // Has at least one adjecent number.
    // return has_adj;

    // Part 2.
    // Must have at least 1 adjacency pair.
    return has_adj && has_2;
}