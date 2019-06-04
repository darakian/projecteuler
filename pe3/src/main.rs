fn main() {
    println!("{}", gcd(18,12));
}

fn gcd(mut n: u32, mut m: u32) -> u32 {
    while n % m != 0 {
        let tmp = m;
        m = n % m;
        n = tmp;
    }
    m
}