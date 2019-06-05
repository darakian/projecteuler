fn main() {
    //println!("Length = {:?}", collatz_length(13).unwrap());
    let mut lengths = [0; 1000001];
    let mut longest = 0;
    for i in (1..=1000000) {
        lengths[i] = collatz_length(i, &lengths).unwrap();
        if lengths[i] > longest{
            longest = lengths[i];
        }
    }
    println!("Longest = {:?}", longest);
}

fn collatz_step(n: usize) -> Option<usize>{
    if n == 1 { return None}
    match n%2 {
        0 => return Some(n/2),
        1 => return Some(3*n+1),
        _ => return None,
    }
}

fn collatz_length(n: usize, lens: &[usize]) -> Result<usize, String>{
    let mut length = 0;
    let mut inner = n;
    while let Some(step) = collatz_step(inner){
        length+=1;
        if step<1000000 && lens[step] != 0 {
            return Ok(lens[step]+length)
        }
        inner=step;
        }
    Ok(length)
}