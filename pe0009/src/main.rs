fn main() {
    for i in 1..=1000{
        for j in 1..=1000{
            for k in 1..=1000{
                if check(i,j,k){
                    println!("{}", i*j*k);
                }
            }
        }
    }
}

fn check(a: u32, b: u32, c: u32) -> bool{
    let ord = a < b && b < c;
    let sum = a+b+c == 1000;
    let prod = a*a + b*b == c*c;
    ord && sum && prod
}