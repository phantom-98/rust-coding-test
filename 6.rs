use std::io;

fn is_armstrong_number(value: &str) -> String {
    let len = value.len() as u32;
    let num: u32 = value.parse().expect("should be number");
    let sum: u32 = value.chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(len))
        .sum();
    
    if num == sum {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn main() {
    let mut list: Vec<String> = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lines: u32 = input_line.trim().parse().expect("Should be number");
    for i in 0..lines {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        
        list.push(is_armstrong_number(input_line.trim()));
    }
    for i in list {
        println!("{}", i);
    }
}