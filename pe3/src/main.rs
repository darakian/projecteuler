fn main() {
    println!("{}", gcd(18,12));
    println!("{:?}", pollard_rho(8051).unwrap());
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
        println!("d: {:?} x: {} y {}", d, x, y);
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