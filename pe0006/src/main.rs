fn main() {
    println!("{}", (1..=100).sum::<u32>().pow(2)-(1..=100).map(|x| x*x).sum::<u32>());
}
