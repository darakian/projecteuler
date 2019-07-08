use std::collections::HashSet;

fn main() {
    println!("Hello, world! {:?}", lexi_permute("321".to_string()));
}

fn lexi_permute(s: String) -> Vec<String>{
    let mut ret = Vec::new();
    let mut chars: HashSet<_> = s.chars().collect();
    for c in chars.iter() {
        let single: HashSet<_> = vec![c.clone()].into_iter().collect::<std::collections::HashSet<_>>();
        let append_strs = chars.difference(&single).collect();
        println!("{:?} {:?}", c, combinations(append_strs));
    }
    ret
}

fn combinations(hesh: HashSet<&char>) -> Vec<&char> {
    //let mut ret = Vec::new();
    // for element in hesh.iter(){

    // }
    hesh.into_iter().collect()
}