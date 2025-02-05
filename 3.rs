use std::io;

fn calculate_area_and_perimeter(dimensions: (u32, u32)) -> (f32, f32) {
    let d = if dimensions.0 < dimensions.1 {
        dimensions.0
    } else {
        dimensions.1
    } as f32;
    const PI: f32 = 3.1415;
    return (PI * d * d / 4.0, PI * d);
}


fn main() {
    let mut list: Vec<(f32, f32)> = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lines: u32 = input_line.trim().parse().expect("Should be number");
    for i in 0..lines {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut inputs = input_line.trim()
                .split_whitespace();
        let num1: u32 = inputs.next().unwrap().parse().expect("Should be number");
        let num2: u32 = inputs.next().unwrap().parse().expect("Should be number");
        
        list.push(calculate_area_and_perimeter((num1, num2)));
    }
    for i in list {
        println!("{:.2} {:.2}", i.0, i.1);
    }
}