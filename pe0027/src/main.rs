fn main() {
    let mut the_combs = uniq_combinations((-999..=999).collect());
    let the_combs: Vec<_> = the_combs.iter().filter(|x| is_prime(x.1)).collect();
    println!("{:?}", the_combs
        .iter()
        .filter(|x| x.0 != 0 && x.1 != 0 && x.0 != x.1)
        .map(|x| (count_primes(x.0, x.1), x.0, x.1))
        .max());
}

fn the_quad(a: i32, b: i32, n: i32) -> i32{
    n*n + a*n + b
}

fn count_primes(a: i32, b:i32) -> u32{
    let mut count = 0;
    let mut n = 0;
    while is_prime(the_quad(a, b, n)){
        //println!("{:?}", the_quad(a, b, n));
        count+=1;
        n+=1;
    }
    //println!("a:{}, b:{} have {}", a, b, count);
    count
}

fn is_prime(n: i32) -> bool{
    if n <= 0 {return false}
    //println!("{:?} is prime?", n);
    if n == 1 || n == 4 || n == 6 || n == 8 {return false}
    if n == 2 || n == 3 || n == 5 || n == 7 {return true}
    if n%2 == 0 {return false}
    let mut i = 3;
    while i*i <= n{
        if n % i == 0{
            return false;
        }
        i+=2;
    }
    true
}

fn uniq_combinations(v: Vec<i32>) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for element1 in v.iter(){
        for element2 in v.iter(){
            out.push((*element1, *element2));
        }
    }
    out
}

fn gcd(mut n: i32, mut m: i32) -> i32 {
    while n % m != 0 {
        let tmp = m;
        m = n % m;
        n = tmp;
    }
    m
}
