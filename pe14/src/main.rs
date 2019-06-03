fn main() {
    //println!("Length = {:?}", collatz_length(13).unwrap());
    let mut longest = 0;
    for i in (1..=1000000).rev() {
        let current = collatz_length(i).unwrap();
        if current > longest{
            longest = current;
        }
    }
    println!("Longest = {:?}", longest);
}

fn collatz_step(n: u64) -> Option<u64>{
    if n == 1 { return None}
    match n%2 {
        0 => return Some(n/2),
        1 => return Some(3*n+1),
        _ => return None,
    }
}

fn collatz_length(n: u64) -> Result<u64, String>{
    let mut length = 0;
    let mut inner = n;
    while let Some(step) = collatz_step(inner){
        length+=1;
        inner=step;
        }
    Ok(length)
}