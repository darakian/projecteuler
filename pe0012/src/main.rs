use std::collections::HashSet;

fn main() {
    let mut i = 0;
    for j in 1.. {
        if divisors(i).len()>=500{
            break;
        }
        i+=j;
    }
    println!("{:?}", i);
}

fn divisors(n: u32) -> HashSet<u32>{
    (1..=(n as f64).sqrt() as u32).filter(|x| n%x==0).flat_map(|x| vec![x, n/x]).collect()
}