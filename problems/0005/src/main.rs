fn main() {
    let result: u64 = [2u64, 3, 5, 7, 11, 13, 17, 19].iter().map(|&p| p.pow(20u64.ilog(p))).product();
    println!("{result}")
}
