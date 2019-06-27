use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("p022_names.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s.retain(|c| c!='(' && c != '\"');
    let mut names: Vec<_> = s.split(",").collect();
    names.sort();
    let mut scores: Vec<_> = names
        .iter()
        .enumerate()
        .map(|x| 
            (x.0 + 1) * name_score(x.1.to_string())).collect();
    println!("{:?}", scores.iter().sum::<usize>());
}

fn name_score(s: String) -> usize {
    s.chars().map(|x| match x{
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
        'I' => 9,
        'J' => 10,
        'K' => 11,
        'L' => 12,
        'M' => 13,
        'N' => 14,
        'O' => 15,
        'P' => 16,
        'Q' => 17,
        'R' => 18,
        'S' => 19,
        'T' => 20,
        'U' => 21,
        'V' => 22,
        'W' => 23,
        'X' => 24,
        'Y' => 25,
        'Z' => 26,
        _ => 0,
    }).sum()
}