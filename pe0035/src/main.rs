use itertools::Itertools;

fn main() {
    let break_point = 100;
    let mut pi = PrimeIter::new();
    let mut primes = vec![];
    //let mut circular_primes = vec![];
    let mut current = 0;
    loop{
        current = pi.next().unwrap();
        if current < break_point{
            primes.push(current);
        } else if current >= break_point{
            break
        }
    }
    for p in primes.into_iter(){
        println!("p:{:?}", p);
        let p_d = digits(p);
        let candidates = p_d.iter().combinations(p_d.len());
        for c in candidates{
            println!("{:?}", vec_to_num(c));
        }
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

fn digits(mut i: u64) -> Vec<u64>{
    let mut out = vec![];
    while i > 0{
        out.push(i % 10);
        i = i/10;
    }
    out
}

fn vec_to_num(input: Vec<&u64>) -> u64{
    let mut factor = 0;
    let mut sum = 0;
    for i in (0..input.len()).rev(){
        sum = sum + input[i]*10_u64.pow(factor);
        factor+=1;
    }
    sum
}
