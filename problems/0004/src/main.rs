fn is_palindrome(n: u32) -> bool {
    let (mut x, mut rev) = (n, 0);
    while x > 0 {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    rev == n
}

fn main() {
    let mut max = 0;
    for i in (100..=999).rev() {
        if i * 999 < max {
            break;
        };
        // All palindromes are multiple of 11
        let (start, step) = if i % 11 == 0 { (999, 1) } else { (990, 11) };
        for j in (100..=start).rev().step_by(step) {
            let mult = i * j;

            if mult < max {
                break;
            };

            if is_palindrome(mult) {
                max = mult;
            }
        }
    }

    println!("{max}");
}
