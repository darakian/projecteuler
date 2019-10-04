fn main() {
    let mut pi = PrimeIter::new();
    let mut primes = vec![];
    let mut circular_primes = vec![];
    let mut current = 0;
    loop{
        current = pi.next().unwrap();
        if 9 < current < 1000000{
            primes.push(current);
        }
    }
    for p in primes.iter(){
        let p_d = digits(p);

    }
}

struct PrimeIter {
    n: u64,
}

impl PrimeIter{
    fn new() -> PrimeIter {
        PrimeIter{n: 2}
    }
}

impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64>{
        if self.n == 2 {
            self.n += 1;
            return Some(2)
        }
        while self.n < u64::max_value(){
            if is_prime(self.n){
                let tmp = self.n;
                self.n+=2;
                return Some(tmp)
            }
            self.n+=2;
        }
    None
    }
}

fn is_prime(n: u64) -> bool{
    if n == 1 {return false}
    let mut i = 3;
    while i*i <= n{
        if n % i == 0{
            return false;
        }
        i+=2;
    }
    true
}

fn digits(mut i: u32) -> Vec<u32>{
    let mut out = vec![];
    while i > 0{
        out.push(i % 10);
        i = i/10;
    }
    out
}

fn possilble_nums(Vec<u32>) -> Vec<u32>{
    
}
