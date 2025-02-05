use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut num: u32 = input_line.trim().parse().expect("Should be number");
    let mut step = 0;
    while num != 1 {
        step += 1;
        if (num % 2 == 0) {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
    }
    println!("{}", step);
}