fn main() {
    let mut fifths = vec![];
    for i in 2..=9999999{
        if i == digits(i)
                .iter()
                .map(|x| x.pow(5))
                .sum(){
                    fifths.push(i);
                    println!("{:?}", i);
        }
    }
    println!("Sum: {:?}", fifths.iter().sum::<u32>());
}

fn digits(mut i: u32) -> Vec<u32>{
    let mut out = vec![];
    while i > 0{
        out.push(i % 10);
        i = i/10;
    }
    out
}