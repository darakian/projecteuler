fn main() {
    //let the_sum: u64 = (1..2000000).filter(|x| is_prime(*x)).sum();
    let an_sum: usize = slow_primes::Primes::sieve(2000000).primes().sum();
    //let the_sum: u64 = (1..2000000).filter(|x| is_prime_miller_rabin(*x)).sum();
    println!("{}", an_sum);
}

// fn is_prime(n: u64) -> bool{
//     if n <= 3 {return true}
//     if n%2 == 0 {return false}
//     let mut i = 3;
//     while i*i <= n{
//         if n%i==0 {
//             return false
//         }
//         i+=2;
//     }
//     true
// }