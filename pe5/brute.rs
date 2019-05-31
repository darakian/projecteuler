fn main() {
    let mut least_common = 0;
    while !correct(least_common) {
        least_common+=20;
    }
    println!("{:?}", least_common);
    println!("{:?}", correct(least_common));
    }

fn correct(n: u32) -> bool{
    if n == 0 {return false}
    for i in 1..20{
        if n%i != 0{
            return false
        }
    }
    true
}
