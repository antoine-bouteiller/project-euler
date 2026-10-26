fn main() {
    let mut sum_square = 0;
    let mut square_sum = 0;
     
    for i in 1..=100 {
        sum_square += i * i;
        square_sum += i;
    }

    square_sum *= square_sum;


    let output = square_sum - sum_square;

    println!("{output}");
}
