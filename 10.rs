use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num: u32 = input_line.trim().parse().expect("Should be number");
    for a in 1..=num/3 {
        for b in a+1..=(num-a)/2 {
            let c = num - a - b;
            if a * a + b * b == c * c {
                println!("{}, {}, {}", a, b, c);
            }
        }
    }
}