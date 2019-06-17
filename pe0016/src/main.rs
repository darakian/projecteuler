fn main() {
    let test: u128 = 1;
    for i in 0..128{
        println!("{:?}", test << i);
    }
}
