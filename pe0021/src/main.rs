use std::collections::HashSet;

fn main() {
    let the_range: Vec::<u32> = (1..=10000).collect();
    let the_pairs: Vec<(u32, u32)> = cross_product(&the_range)
        .into_iter()
        .filter(|x| dee(x.0) == x.1 && dee(x.1)== x.0 && x.0 != x.1)
        .collect();
    //println!("{:?}", the_pairs);
    let (left, right): (HashSet::<u32>, HashSet::<u32>) = the_pairs.into_iter().unzip();
    let amicables: HashSet::<_> = left.union(&right).collect();
    println!("{:?}", amicables.into_iter().sum::<u32>());


    //println!("{:?}, {:?}", dee(220), dee(284));
}

fn divisors(n: u32) -> HashSet<u32>{
    (1..(n as f64).sqrt() as u32).filter(|x| n%x==0).flat_map(|x| vec![x, n/x]).collect()
}

fn dee(n: u32) -> u32 {
    divisors(n).iter().sum::<u32>() as u32 - n
}

fn cross_product(vec: &Vec<u32>) -> Vec<(u32, u32)>{
    let mut pairs = Vec::new();
    for first_token in vec.iter().enumerate(){
        let index = first_token.0;
        let val = first_token.1;
        for second_val in &vec[index..]{
            //println!("{:?}, {:?}", second_val, index);
            pairs.push((val.clone(), second_val.clone()));
        }
    }
    pairs
}