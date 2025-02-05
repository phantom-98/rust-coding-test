
## 1. Testing Rust development environment.
If you didn’t install Rust development environment, please install `rustup` and check the `rustc` and `cargo` versions.
And then create a new cargo project and print “Hello, world!”. That’s all for now.

## 2. Rust CLI program (intermediate)
You should generate random number(any value is OK) using `rand::Rng` module between(including) 2 input numbers.
Input will finish if the 2 value is both `0`.
You should use `loop` function.
```
Input:
2 10
3 7
100 200
0 0

Output:
3
6
100
```

## 3. Function and Data type
You need to implement a function in Rust. This function takes a tuple representing the width and height of a rectangle and returns a tuple containing the area and perimeter of the largest circle inside the rectangle.
```
// Function to calculate the area and perimeter of a rectangle
fn calculate_area_and_perimeter(dimensions: (u32, u32)) -> (u32, u32) {

}
```
```
Input:
3 //test count
2 5
3 3
5 4

Output:
12.56 12.56
28.26 18.84
50.24 25.12
```

## 4. Understanding Ownership
You need to implement two functions that demonstrate an understanding of ownership and borrowing:
1. **append_to_vector**: This function takes a mutable reference to a vector of integers and an integer value. It appends the value to the vector.
2. **calculate_average**: This function takes a reference to a vector of integers and returns the average of the values in the vector.
```
// Function to append a value to a vector
fn append_to_vector(vec: &mut Vec<i32>, value: i32) {

}
// Function to calculate the average of values in a vector
fn calculate_average(vec: &Vec<i32>) -> f64 {

}
```
Testing code:
```
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
```

## 5. Struct and Method Syntax
You need to implement a Student struct in Rust with methods that operate on instances of this struct:
1. **new**: This method takes a name (`string`), age (`u32`), and grades (vector of `u32`) and returns a new Student instance.
2. **average_grade**: This method calculates and returns the average grade of the student.
3. **add_grade**: This method takes a grade (`u32`) and adds it to the student's grades.

```
// Define the Student struct
struct Student {
    name: String,
    age: u32,
    grades: Vec<u32>,
}
impl Student {
    // Method to create a new Student instance
    fn new(name: &str, age: u32, grades: Vec<u32>) -> Student {

    }
    // Method to calculate the average grade of the student
    fn average_grade(&self) -> f64 {

    }
    // Method to add a grade to the student's grades
    fn add_grade(&mut self, grade: u32) {

    }
}
```
Testing code:
```
fn main() {
    // Test new
    let mut student1 = Student::new("Alice", 20, vec![85, 90, 78]);
    assert_eq!(student1.name, "Alice");
    assert_eq!(student1.age, 20);
    assert_eq!(student1.grades, vec![85, 90, 78]);
    let student2 = Student::new("Bob", 22, vec![70, 80, 90]);
    assert_eq!(student2.name, "Bob");
    assert_eq!(student2.age, 22);
    assert_eq!(student2.grades, vec![70, 80, 90]);
    // Test average_grade
    assert_eq!(student1.average_grade(), 84.33333333333333);
    assert_eq!(student2.average_grade(), 80.0);
    // Test add_grade
    student1.add_grade(95);
    assert_eq!(student1.grades, vec![85, 90, 78, 95]);
    println!("All tests passed!");
}
```

## 6. Armstrong number
An Armstrong number is a number that is the sum of its own digits each raised to the power of the number of digits. For example:

• `9` is an Armstrong number, because `9 = 9^1 = 9`

• `10` is not an Armstrong number, because `10 != 1^2 + 0^2 = 1`

• `153` is an Armstrong number, because: `153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153`

• `154` is not an Armstrong number, because: `154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190`

Write some code to determine whether a number is an Armstrong number.
```
Input:
4 //test count
9
10
153
154

Output:
Yes
No
Yes
No
```

## 7. All your Base
Convert a sequence of digits in one base, representing a number, into a sequence of digits in another base, representing the same number.

**Note**

Try to implement the conversion yourself. Do not use something else to perform the conversion for you.
```
Input:
3 // test count
101010 2
1120 3
42 10

Output:
42
42
42
```

## 8. Collatz Conjecture
The Collatz Conjecture or `3x+1` problem can be summarized as follows:

Take any positive integer `n`. If `n` is even, divide `n` by `2` to get `n / 2`. If `n` is odd, multiply `n` by `3` and add `1` to get `3n + 1`. Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach `1` eventually. Given a number `n`, return the number of steps required to reach `1`.

**Examples**

Starting with `n = 12`, the steps would be as follows:

```
12
6
3
10
5
16
8
4
2
1
```
Resulting in 9 steps. So for input `n = 12`, the return value would be `9`.

## 9. Acronym
Convert a phrase to its acronym. Techies love their TLA (Three Letter Acronyms)!
Help generate some jargon by writing a program that converts a long name like Portable Network Graphics to its acronym (PNG). Punctuation is handled as follows: hyphens are word separators (like whitespace); all other punctuation can be removed from the input. For example:
```
Input:
3 // test count
As Soon As Possible
Liquid-crystal display
Thank George It's Friday!

Output:
ASAP
LCD
TGIF
```

## 10. Pythagorean Triplet
A Pythagorean triplet is a set of three natural numbers, `{a, b, c}`, for which,

`a² + b² = c²`

and such that,

`a < b < c`

For example,

`3² + 4² = 5²`.

Given an input integer `N`, find all Pythagorean triplets for which `a + b + c = N`.

For example, with `N = 1000`, there is exactly one Pythagorean triplet for which `a + b + c = 1000`: `{200, 375, 425}`.