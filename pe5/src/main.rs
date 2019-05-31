use gcd::Gcd;

fn main() {
    let the_value = least_common(1, 
        least_common(2,
        least_common(3,
        least_common(4,
        least_common(5,
        least_common(6,
        least_common(7,
        least_common(8,
        least_common(9,
        least_common(10,
        least_common(11,
        least_common(12,
        least_common(13,
        least_common(14,
        least_common(15,
        least_common(16,
        least_common(17,
        least_common(18,
        least_common(19,20,
            )))))))))))))))))));
    println!("{}", the_value);
}

fn least_common(n: u32, m: u32) -> u32{
    (n*m) / n.gcd(m)
}
