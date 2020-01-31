fn main() {
    let pi = PrimeIter::new();
    let vec: Vec<u64> = pi.take(11).collect();
    println!("{:?}", vec.iter().sum::<u64>());
}

struct PrimeIter{
    current: u64
}

impl PrimeIter {
    fn new() -> Self{
        PrimeIter{current:1}
    }
}

impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64>{
        self.current+=2;
        while !is_truncatable_prime(self.current){
            self.current+=2
        }
        Some(self.current)
    }
}

fn is_prime(n: u64) -> bool{
    if n==1 {return false}
    let mut i = 2;
    while i*i <= n{
        if n % i == 0{
            return false;
        }
        i+=1;
    }
    true
}

fn is_truncatable_prime(n: u64) -> bool{
    if n < 10 {
        return false
    }
    is_t_l_to_r(n) && is_t_r_to_l(n)
}

fn is_t_r_to_l(mut n: u64) -> bool{
    while n > 0 {
        if !is_prime(n){
            return false
        }
        //println!("{:?}, {}", n, is_prime(n));
        n = n/10;
    }
    true
}

fn is_t_l_to_r(n: u64) -> bool{
    let digits: Vec<_> = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut tmp = 0;
    for element in digits.iter().enumerate(){
        tmp += element.1 * 10u32.pow((digits.len() - element.0 - 1) as u32);
        if !is_prime(n-tmp as u64){
            return false
        }
        //println!("{:?} {} {} {}", element.0, element.1, n-tmp as u64, is_prime(n-tmp as u64));
    }
    true
}