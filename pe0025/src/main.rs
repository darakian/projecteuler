fn main() {
    let mut i = 0;
    let mut j = 1;
    while digits(j) < 1000{
        j = j + i;
        i = j;
        println!("i:{} j:{} digits:{}", i, j, digits(j));
    }
    println!("Hello, world! {}", j);
}

fn digits(n :u64) -> usize{
    n.to_string().len()
}