use std::char;

struct BigBaduInt {
    digits: Vec<char>,
}

impl BigBaduInt{
    pub fn new(value: u32) -> Self {
        let mut b = BigBaduInt{digits: vec![]};
        b.add(value);
        b
    }

    pub fn new_from_str(value: &str) -> Self {
        let mut b = BigBaduInt{digits: vec![]};
        b.add_str(value);
        b
    }

    pub fn add(&mut self, value: u32) -> (){
        self.add_str(&value.to_string())
    }

    pub fn add_str(&mut self, value: &str) -> (){
        let mut carry = None;

        for (index, c) in value.chars().rev().enumerate(){
            carry = self.add_place(index, c, carry);
        }
        if carry.is_some(){
            self.add_str(&(carry.unwrap().to_string() + &"0".repeat(value.len())));
            
        }
    }

    pub fn mul(&mut self, value: u16) -> (){
        let current = self.value();
        for i in 1..value{
            self.add_str(&current);
        }
    }

    fn add_place(&mut self, index: usize, num: char, carry: Option<char>) -> Option<char>{
        if let Some(x) = self.digits.get(index){
            let value = x;
            let ival = value.to_string().parse::<u32>().unwrap();
            let inum = num.to_string().parse::<u32>().unwrap();
            let mut icarry = 0;
            if carry.is_some() {
                icarry = carry.unwrap().to_string().parse::<u32>().unwrap_or(0);
            }
            let new_val = ival+inum+icarry;
            if new_val <10{
                let cnew_val = char::from_digit(new_val, 10).unwrap();
                self.digits[index] = cnew_val;
                return None
            } else {
                let ones = new_val % 10;
                let tens = (new_val - ones)/10;
                let cnew_val = char::from_digit(ones, 10).unwrap();
                self.digits[index] = cnew_val;
                return Some(char::from_digit(tens, 10).unwrap())
            }

        }
        else {
            self.digits.push(num);
            return None
        };
    }

    pub fn value(&self) -> String{
        self.digits.iter().rev().collect()
    }
}

fn main() {
    let mut fib = BigBaduInt::new(1);
    let mut prior = fib.value();
    fib.add(1);
    let mut index = 3;
    while fib.value().len() < 1000 {
        // println!("Index: {:?} Value: {:?}", index, fib.value());
        let current = fib.value();
        fib = BigBaduInt::new_from_str(&current);
        fib.add_str(&prior);
        prior = current;
        index+=1;
    }
    println!("{:?}", fib.value());
    println!("{:?}", fib.value().len());
    println!("{:?}", index);
}
