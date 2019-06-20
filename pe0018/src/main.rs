use std::fs::File;
use std::io::Read;
use std::cmp;

fn get_level(index: usize) -> usize{
    let mut candidate_level = tri_num(1).unwrap();
    let mut level = 1;
    while index > candidate_level as usize {
        level+=1;
        candidate_level = tri_num(level).unwrap();
    }
    //println!("index:{:?}, level {:?}", index, level);
    level as usize
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

fn left_child_index(index: usize) -> usize{
    get_level(index)+index
}

fn right_child_index(index: usize) -> usize{
    get_level(index)+index+1
}

fn main() {
    let mut file = File::open("nums67.txt").unwrap();
    let mut nums = String::new();
    file.read_to_string(&mut nums).unwrap();
    let mut tree_vec = vec![0];
    for token in nums.split_whitespace(){
        let number = token.parse::<u32>().unwrap();
        tree_vec.push(number);
    }
    let mut path = Vec::new();
    let mut index = 1;
    // while index < tree_vec.len() as u32{
    //     path.push(tree_vec[index as usize]);
    //     let left_c_i = left_child_index(index);
    //     let right_c_i = right_child_index(index);
    //     let next = best_next(left_c_i, right_c_i, &tree_vec);
    //     if next == None {break}
    //     else {index = next.unwrap()}
    // }
    println!("{:?}", max_path_sum(1, tree_vec[1], &tree_vec));

    println!("{:?}.len({:?}).sum({:?})", path, path.len(), path.iter().sum::<u32>());    
}

fn best_next(left: u32, right: u32, vec: &Vec<u32>) -> Option<u32>{
    let max = vec.len() as u32;
    if left > max && right < max{
        return Some(right)
    } else if left < max && right > max{
        return Some(left)
    } else if left > max && right > max{
        return None
    } else if left < max && right < max{
        let l_v = vec[left as usize];
        let r_v = vec[right as usize];
        if l_v > r_v {
            return Some(left)
        } else {
           return Some(right) 
        }
    }
    None
}

fn fold(index: u32, val: u32, vec: &Vec<u32>) -> Option<((usize, u32), (usize, u32))> {
    let max = vec.len();
    let left = left_child_index(index as usize);
    let right = right_child_index(index as usize);
    if left > max || right > max {return None}
    Some((
        (right, vec[right as usize]+val),
        (left, vec[left as usize]+val)
        ))
}

fn max_path_sum(index: usize, val: u32, vec: &Vec<u32>) -> u32 {
    match fold(index as u32, val, vec){
        None => {return val},
        Some(pair) => {
            return cmp::max(
                max_path_sum((pair.0).0, (pair.0).1, vec),
                max_path_sum((pair.1).0, (pair.1).1, vec)
            )}
    }
}