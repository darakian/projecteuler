fn main() {
    println!("{:?}", n_chose_k(40,20).unwrap());
}

fn n_chose_k(n: u128, k: u128) -> Option<u128> {
    if k>n || k == 0 {return None}
    Some((1..=k).fold( //https://en.wikipedia.org/wiki/Binomial_coefficient#Multiplicative_formula
    1, |acc, i| 
    acc*(n+1-i)/i
    )) 
}