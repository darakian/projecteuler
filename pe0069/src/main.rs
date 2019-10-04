use std::collections::HashSet;

fn main() {
    let test = 9;
    println!("prime divs: {:?}", prime_divisors(test));
    println!("totient: {:?}", totient(test));
}

fn totient(n: u64) -> u64{
    let mut prod = 1.0;
    let f_n = n as f64;
    for div in prime_divisors(n){
        if div == 1 {continue}
        println!(">> (1.0 - 1.0/{})", div);
        prod = prod*(1.0-1.0/div as f64);
    }
    (prod*f_n) as u64
}

fn prime_divisors(n: u64) -> HashSet<u64>{
    (1..=n)
        .filter(|x| is_prime(*x))
        .filter(|x| n%x==0)
        .collect()
}

fn is_prime(n: u64) -> bool{
    if n == 1 {return false}
    if (n%2 == 0) && (n != 2) {return false}
    let mut i = 3;
    while i*i <= n{
        if n % i == 0{
            return false;
        }
        i+=2;
    }
    true
}