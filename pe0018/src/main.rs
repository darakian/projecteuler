use std::fs::File;
use std::io::Read;

fn get_level(index: u32) -> u32{
    let mut candidate_level = tri_num(1).unwrap();
    let mut level = 1;
    while index > candidate_level {
        level+=1;
        candidate_level = tri_num(level).unwrap();
    }
    //println!("index:{:?}, level {:?}", index, level);
    level
}

fn tri_num(i: u32) -> Option<u32>{
    n_choose_k(i+1, 2)
}

fn n_choose_k(n: u32, k: u32) -> Option<u32> {
    if k>n || k == 0 {return None}
    Some((1..=k).fold( //https://en.wikipedia.org/wiki/Binomial_coefficient#Multiplicative_formula
    1, |acc, i| 
    acc*(n+1-i)/i
    )) 
}

fn left_child_index(index: u32) -> u32{
    get_level(index)+index
}

fn right_child_index(index: u32) -> u32{
    get_level(index)+index+1
}

fn main() {
    let mut file = File::open("nums.txt").unwrap();
    let mut nums = String::new();
    file.read_to_string(&mut nums).unwrap();
    let mut tree_vec = vec![0];
    for token in nums.split_whitespace(){
        let number = token.parse::<u32>().unwrap();
        tree_vec.push(number);
    }
    let mut path = Vec::new();
    let mut index = 1;
    while index <= tree_vec.len() as u32{
        path.push(tree_vec[index as usize]);
        index = right_child_index(index);
    }
    println!("{:?}", path);
    // for i in 1..=10{
    //     print!("i = {:?} ", i);
    //     print!("tri(i) = {:?} ", tri_num(i));
    //     println!("level(i) = {:?}", get_level(i));
    // }
    
}
