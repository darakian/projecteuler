use itertools::Itertools;

fn main() {
    let mut largest = 0;
    let combs: Vec<(_, _)> = (100..999).tuple_combinations().unique().collect();
    println!("{:?}", combs.len());
    for (x, y) in (100..999).tuple_combinations() {
        if x*y > largest && is_palindrome(x*y){
            largest = x*y
        }
    }
    println!("largest = {}", largest);
}

fn is_palindrome(n: u32) -> bool{
    n == reverse_int(n)
}

fn reverse_int(mut n: u32) -> u32{
    let mut rev = 0;
    while n != 0 {
        rev = rev*10 + n%10;
        n = n/10;
    }
    rev
}