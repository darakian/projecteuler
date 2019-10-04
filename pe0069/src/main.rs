use std::collections::HashSet;

fn main() {
    let mut max_tot = 1.0;
    let mut max_n = 1;
    (1..=10)
        .for_each(|x| {
            let tmp = n_over_tot(x);
            if tmp>max_tot{
                    max_tot = tmp;
                    max_n = x;
                }
            });
    //println!("n:{}, n/tot:{}", max_n, max_tot);
}

fn n_over_tot(n: u64) -> f64{
    let tot = totient(n);
    println!("n:{} tot:{:?}", n, tot);
    n as f64/tot as f64
}

fn totient(n: u64) -> u64{
    if n == 1 {return 1}
    let mut prod = 1.0;
    let f_n = n as f64;
    for div in prime_divisors(n){
        if div == 1 {continue}
        prod = prod*(1.0-1.0/div as f64);
    }
    //println!(">> n:{} tot:{:?}", n, (prod*f_n));
    (prod*f_n) as u64
}

fn prime_divisors(n: u64) -> HashSet<u64>{
    (1..=n)
        .filter(|x| n%x==0)
        .filter(|x| is_prime(*x))
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