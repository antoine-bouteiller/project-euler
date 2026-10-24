/// Randomised per visitor on https://projecteuler.net/register; update if the page changes.
const N: u64 = 895_000;

fn main() {
    let mut sum: u64 = 0;
    for i in 1..=N {
        if i % 2 == 1 {
            sum += i * i;
        }
    }
    println!("{sum}");
}
