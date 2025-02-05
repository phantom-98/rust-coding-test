// Function to append a value to a vector
fn append_to_vector(vec: &mut Vec<i32>, value: i32) {
    vec.push(value);
}
// Function to calculate the average of values in a vector
fn calculate_average(vec: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for i in vec {
        sum += i;
    }
    return sum as f64 / 4.0;
}

fn main() {
    // Test append_to_vector
    let mut vec1 = vec![1, 2, 3];
    append_to_vector(&mut vec1, 4);
    assert_eq!(vec1, vec![1, 2, 3, 4]);
    let mut vec2 = vec![10, 20, 30];
    append_to_vector(&mut vec2, 40);
    assert_eq!(vec2, vec![10, 20, 30, 40]);
    // Test calculate_average
    let vec3 = vec![1, 2, 3, 4];
    assert_eq!(calculate_average(&vec3), 2.5);
    let vec4 = vec![10, 20, 30, 40];
    assert_eq!(calculate_average(&vec4), 25.0);
    println!("All tests passed!");
}