fn main() {
    println!("Hello, world! {:?}", lexi_permute("321".to_string()));
}

fn lexi_permute(s: String) -> Vec<Vec<char>>{
    let mut ret = Vec::new();
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    for c in chars.iter() {

    }
}