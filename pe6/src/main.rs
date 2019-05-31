fn main() {
    let square_of_sums = (1..=100).sum::<u32>().pow(2);
    let sum_of_squares = (1..=100).map(|x| x*x).sum::<u32>();
    println!("{}", square_of_sums-sum_of_squares);
}
