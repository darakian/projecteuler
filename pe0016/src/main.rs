use std::char;

struct BigBaduInt {
    digits: Vec<char>,
}

impl BigBaduInt{
    pub fn new() -> Self {
        BigBaduInt{digits: vec![]}
    }

    pub fn add(&mut self, value: &str) -> (){
        let mut carry = None;

        for (index, c) in value.chars().rev().enumerate(){
            carry = self.add_place(index, c, carry);
        }
        if carry.is_some(){
            self.add(&(carry.unwrap().to_string() + &"0".repeat(value.len())));
            
        }
    }

    pub fn mul(&mut self, value: u16) -> (){
        let current = self.value();
        for i in 1..value{
            self.add(&current);
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
    let mut two_to_1000 = BigBaduInt::new();
    two_to_1000.add("2");
    for _i in 1..1000{
        two_to_1000.mul(2);
    }
    let sum: u32 = two_to_1000.value().chars().map(|x| x.to_string().parse::<u32>().unwrap()).sum();
    println!("{:?}", sum);
}

