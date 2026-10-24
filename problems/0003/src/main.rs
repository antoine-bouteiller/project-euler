fn is_prime(input: u64) -> bool {
    for i in 2..input.isqrt() {
        if input.is_multiple_of(i) {
            return false;
        }
    }
    true
}


fn main() {
    let input: u64 = 600851475143;
    for i in (1..input.isqrt()).rev() {
        if input.is_multiple_of(i) && is_prime(i) {
            println!("{i}");
            return;
        }
    }
}
