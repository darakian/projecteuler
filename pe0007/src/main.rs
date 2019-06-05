fn main() {
    println!("Hello, world!");
}

fn is_prime(n: u64) -> bool{
    if n <= 3 {return true}
    if n%2 == 0 {return false}
    let mut i = 3;
    while i*i <= n{
        if n%i==0 {
            return false
        }
        i+=2;
    }
    true
}