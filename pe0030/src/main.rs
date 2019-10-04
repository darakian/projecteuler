fn main() {
    let mut fifths = vec![];
    for mut i in 10000..=99999 {
        let original = i;
        let ones: u32 = i % 10;
        i = i/10;
        let tens: u32 = i % 10;
        i = i/10;
        let hundreds: u32 = i % 10;
        i = i/10;
        let thous: u32 = i % 10;
        i = i/10;
        let ten_thous: u32 = i % 10;
        let fifth_power = ones.pow(5) + tens.pow(5) + hundreds.pow(5) + thous.pow(5) + ten_thous.pow(5);
        if original == fifth_power {
            fifths.push(original);
            println!("{:?}", original);
        }
    }
    println!("{:?}", fifths.iter().sum::<u32>());
}
