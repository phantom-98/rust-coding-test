use std::io;

fn get_number_base_10(value: &str, base: u32) -> u32 {
    let len = value.len() as u32;
    let mut sum = 0;
    for i in 0..len {
        let c = value.chars().nth(i as usize).unwrap();
        sum += c.to_digit(10).unwrap() as u32 * base.pow(len - 1 - i);
    }
    return sum;
}

fn main() {
    let mut list: Vec<u32> = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lines: u32 = input_line.trim().parse().expect("Should be number");
    for i in 0..lines {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut inputs = input_line.trim().split_whitespace();
        let value = inputs.next().unwrap();
        let base: u32 = inputs.next().unwrap().parse().unwrap();
        
        list.push(get_number_base_10(value, base));
    }
    for i in list {
        println!("{}", i);
    }
}