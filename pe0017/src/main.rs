fn main() {
    let mut letters = Vec::new();
    for i in 1..=1000{
        letters.append(&mut n_to_words(i));
    }
    println!("{:?}", letters.len());
    // let test = 999;
    // for i in 1..=20{
    //     for token in n_to_words(i){
    //         print!("{:?}", token);
    //     }
    //     println!(" ");
    //     println!("{}", n_to_words(i).len());
    // }
}

fn n_to_words(n: u16) -> Vec<char>{
    let mut weird = 0;
    let mut ones = n % 10; 
    let mut tens = (n % 100) - ones;
    let hundreds = (n % 1000) - tens - ones;
    let thousands = (n % 10000) - hundreds - tens - ones;
    if n % 100 < 20 && n%100 > 10 {
        ones = 0;
        tens = 0;
        weird = n % 100;
    }
    let th_word = match thousands {
        1000 => "One Thousand",
        2000 => "Two Thousand",
        3000 => "Three Thousand",
        4000 => "Four Thousand",
        5000 => "Five Thousand",
        6000 => "Six Thousand",
        7000 => "Seven Thousand",
        8000 => "Eight Thousand",
        9000 => "Nine Thousand",
        _ => "",
    };
    let h_word = match hundreds {
        100 => "One Hundred",
        200 => "Two Hundred",
        300 => "Three Hundred",
        400 => "Four Hundred",
        500 => "Five Hundred",
        600 => "Six Hundred",
        700 => "Seven Hundred",
        800 => "Eight Hundred",
        900 => "Nine Hundred",
        _ => "",
    };
    let t_word = match tens {
        10 => "Ten",
        20 => "Twenty",
        30 => "Thirty",
        40 => "Forty",
        50 => "Fifty",
        60 => "Sixty",
        70 => "Seventy",
        80 => "Eighty",
        90 => "Ninety",
        _ => "",
    };
    let o_word = match ones{
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => "",
    };
    let w_word = match weird{
        11 => "Eleven",
        12 => "Twelve",
        13 => "Thirteen",
        14 => "Fourteen",
        15 => "Fifteen",
        16 => "Sixteen",
        17 => "Seventeen",
        18 => "Eighteen",
        19 => "Nineteen",
        _ => "",
    };
    //println!("{} {} {} {}",thousands, hundreds, tens, ones);
    //println!("{} {} {} {} {}", th_word, h_word, t_word, o_word, w_word);
    let mut ret_string=th_word.to_owned()+&h_word.to_owned()+&t_word.to_owned()+&o_word.to_owned()+&w_word;
    if (th_word != "" || h_word != "") && (t_word != "" || o_word != "" || w_word != ""){
        ret_string = ret_string+"and"
    }
    ret_string.chars().filter(|x| !x.is_whitespace()).collect()
}
