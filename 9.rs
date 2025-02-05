use std::io;

fn main() {
    let mut list: Vec<String> = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num: u32 = input_line.trim().parse().expect("Should be number");
    for i in 0..num {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut abbr = String::new();
        for word in input_line.trim().replace("-", " ").split_whitespace() {
            abbr.push(word.to_uppercase().chars().nth(0).unwrap());
        }
        list.push(abbr);
    }
    for i in list {
        println!("{}", i);
    }
}