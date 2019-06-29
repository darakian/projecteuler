use std::collections::HashSet;

fn main() {
    let sums: HashSet<_> = uniq_combinations(abundant_numbers(28123)).iter().map(|x| x.0+x.1).collect();
    println!("{:?}", (1..=28123).filter(|x| !sums.contains(x)).sum::<u32>());
}

fn divisors(n: u32) -> HashSet<u32>{
    let mut divs: HashSet<u32> = (1..=(n as f64).sqrt() as u32).filter(|x| n%x==0).flat_map(|x| vec![x, n/x]).collect();
    divs.remove(&n);
    divs
}

fn abundant_numbers(upper_limit: u32) -> Vec<u32>{
    (1..=upper_limit).filter(|x| divisors(*x).iter().sum::<u32>() > *x).collect()
}

// fn can_be_sum(n: u32, sums: Vec<u32>) -> bool{
//     let combs = uniq_combinations(abundant_numbers(n));
//     let pairs: Vec<_> = combs.iter().filter(|x| x.0+x.1==n).collect();
//     pairs.len() > 0
// }

fn uniq_combinations(v: Vec<u32>) -> HashSet<(u32, u32)> {
    let mut out = HashSet::new();
    for element1 in v.iter().enumerate(){
        for element2 in v[element1.0..].iter(){
            out.insert((*element1.1, *element2));
        }
    }
    out
}
