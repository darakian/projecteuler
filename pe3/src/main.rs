fn main() {
    // println!("{:?}", pollard_rho(10402).unwrap());
    // println!("{:?}", pollard_rho(1486).unwrap());
    println!("{:?}", pollard_rholler(112));
}

fn gcd(mut n: u32, mut m: u32) -> u32 {
    while n % m != 0 {
        let tmp = m;
        m = n % m;
        n = tmp;
    }
    m
}

fn pollard_rho(n: u32) -> Option<u32>{
    let mut d = 1;
    let mut x = 2;
    let mut y = 2;
    while d == 1{
        x = pollard_rho_helper(x, n);
        y = pollard_rho_helper(pollard_rho_helper(y, n), n);
        d = gcd((x as i64-y as i64).abs() as u32, n);
    }
    if d == n{
        return None
    } else {
        return Some(d)
    }

    fn pollard_rho_helper(x: u32, n: u32) -> u32{
        (x*x+1)%n
    }
}

fn pollard_rholler(mut n: u32) -> Vec<u32>{
    let mut prime_factors = Vec::new();
    if is_prime(n) {prime_factors.push(n); return prime_factors}
    while !is_prime(n){
        println!("{:?}", n);
        if n.is_power_of_two(){
            while n>=2{
                prime_factors.push(2);
                n = n/2;
            }
        } else {
            let p = pollard_rho(n).unwrap();
            prime_factors.push(p);
            n = n/p;
        }
    }
    prime_factors

}

fn is_prime(n: u32) -> bool{
    let mut i = 2;
    while i*i < n{
        if n % i == 0{
            return false;
        }
        i+=1;
    }
    true
}