fn main() {
    let an_sum: u64 = (1..=2000000).filter(|x| is_prime(*x)).sum();
    println!("{}", an_sum);
}

fn is_prime(n: u64) -> bool{
    if i == 1 {return false}
    let mut i = 2;
    while i*i <= n{
        if n % i == 0{
            return false;
        }
        i+=1;
    }
    true
}