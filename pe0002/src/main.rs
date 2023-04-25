fn main() {
    let fibber = Fibiter::new();
    let mut the_sum = 0;
    for current in fibber.into_iter(){
        if current >= 4000000{
            break;
        }
        if current%2 == 0{
            the_sum+=current;
        }
    }
    println!("{}", the_sum);
}

fn fib_next(n: u64, m: u64) -> u64{
    return n+m
}

struct Fibiter {
    first: u64,
    second: u64,
}

impl Fibiter {
    fn new() -> Fibiter {
        Fibiter { first: 1, second: 1 }
    }
}


impl Iterator for Fibiter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = fib_next(self.first, self.second);
        self.first = self.second;
        self.second = ret;
        Some(ret)
    }
}
