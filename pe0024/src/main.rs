fn main() {
    let lots = combinations(vec!['0','1','2','3','4','5','6','7','8','9']);
    println!("{:?}", lots[999999]);
}

fn combinations(invec: Vec<char>) -> Vec<String> {
    let mut ret = Vec::new();
    for c in invec.into_iter(){
        if ret.len() == 0{
            ret.push(c.to_string());
        } else {
            let t_vec: Vec<String> = ret.drain(..).collect();
            for s in t_vec.into_iter(){
                ret.extend(all_strs(s, c).drain(..));
            }
        }
    }
    ret.sort();
    ret
}

fn all_strs(invec: String, new_char: char) -> Vec<String>{
    let mut ret = Vec::new();
    for i in 0..=invec.len(){
        let mut t_vec = invec.clone();
        t_vec.insert(i, new_char);
        ret.push(t_vec);
    }
    ret
}