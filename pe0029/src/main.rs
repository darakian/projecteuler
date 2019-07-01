use std::collections::HashSet;

fn main() {
    let mut the_nums = HashSet::new();
    for a in 2..=100{
        for b in 2..=100{
            the_nums.insert(pow_I_guess(a, b));
        }
    }
    println!("Hello, world! {}", the_nums.len());
}

fn pow_I_guess(a: u32, b: u32) -> Vec<u8>{
    let mut ret = Vec::new();
    let mut i = 0;
    while i < b{
        for _j in 1..=a{
            ret.push(1);
        }
        i+=1;
    }
    ret
}
