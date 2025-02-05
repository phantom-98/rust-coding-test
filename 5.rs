// Define the Student struct
struct Student {
    name: String,
    age: u32,
    grades: Vec<u32>,
}
impl Student {
    // Method to create a new Student instance
    fn new(name: &str, age: u32, grades: Vec<u32>) -> Student {
        Student {name: name.to_string(), age, grades}
    }
    // Method to calculate the average grade of the student
    fn average_grade(&self) -> f64 {
        let mut sum = 0;
        for i in &self.grades {
            sum += i;
        }
        return sum as f64 / self.grades.len() as f64;
    }
    // Method to add a grade to the student's grades
    fn add_grade(&mut self, grade: u32) {
        self.grades.push(grade);
    }
}

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