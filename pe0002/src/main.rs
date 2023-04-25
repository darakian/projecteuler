fn main() {
    let mut the_sum = 0;
    let mut i = 1;
    let mut j = 1;
    while let current = fib_next(i, j){
        if current >= 4000000{
            break;
        }
        if current%2 == 0{
            the_sum+=current;
        }
        i = j;
        j = current;
    }
    println!("{}", the_sum);
}

fn fib_next(n: u32, m: u32) -> u32{
    return n+m
}