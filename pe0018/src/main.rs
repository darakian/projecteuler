use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("nums.txt").unwrap();
    let mut nums = String::new();
    file.read_to_string(&mut nums);
    let mut tree_vec = Vec::new();
    nodes.push_back(&mut first);
    for token in nums.split_whitespace(){
        let number = token.parse::<u32>().unwrap();
        tree_vec.push(number);
        
    }
}
