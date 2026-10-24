fn main() {
    let mut sum: u64 = 0;
    let mut prev1: u64 = 1;
    let mut prev2: u64 = 2;

    while prev1 < 4_000_000 {
        let next: u64 = prev1 + prev2;

        if prev1.is_multiple_of(2) {
            sum += prev1;
        }
        prev1 = prev2;
        prev2 = next;
    }

    println!("{sum}")
    
}
