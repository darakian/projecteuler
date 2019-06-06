fn main() {
    let mut primes = n_primes(10001);
    println!("{:?}", primes.pop().unwrap());
}

fn is_prime(n: u64) -> bool{
    let mut i = 2;
    while i*i <= n{
        if n % i == 0{
            return false;
        }
        i+=1;
    }
    true
}

fn n_primes(n: u64) -> Vec<u64>{
    let mut ret_vec = vec![];
    if n == 0 {return ret_vec}
    ret_vec.push(2);
    let mut iter = 3;
    while ret_vec.len() < n as usize{
        if is_prime(iter){
            ret_vec.push(iter);
        }
        iter+=2;
    }
    ret_vec
}