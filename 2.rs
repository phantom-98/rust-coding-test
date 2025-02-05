use std::io;
use rand::Rng;

fn main() {
    let mut list: Vec<i32> = Vec::new();
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line);
        let mut inputs = input_line.trim()
                .split_whitespace();
        let num1: i32 = inputs.next().unwrap().parse().expect("Should be number");
        let num2: i32 = inputs.next().unwrap().parse().expect("Should be number");
        if num1 == 0 && num2 == 0 {
            break;
        }
        let mut rng = rand::thread_rng();
        let num;
        if num1 <= num2 {
            num = rng.gen_range(num1..=num2);
        } else {
            num = rng.gen_range(num2..=num1);
        }
        list.push(num);
    }
    for i in list {
        println!("{}", i);
    }
}