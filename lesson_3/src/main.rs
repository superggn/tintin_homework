#[allow(unused)]

fn main() {
    let xiaoming = Student {
        name: "xiaoming".to_string(),
    };
    let mut math_course = Course {
        name: "math".to_string(),
        students: vec![&xiaoming],
    };
    let mut class_A = Class {
        name: "class_A".to_string(),
        students: vec![&xiaoming],
    };
    let mut Society_A = Society {
        name: "Society_A".to_string(),
        students: vec![&xiaoming],
    };
    println!("{:?}", math_course);
    math_course.name = "math_course".to_string();
    println!("{:?}", math_course);
    let xiaohong = Student {
        name: "xiaohong".to_string(),
    };
    math_course.students.push(&xiaohong);
    println!("{:?}", math_course);
}

#[derive(Debug)]
struct Class<'a> {
    name: String,
    students: Vec<&'a Student>,
}
#[derive(Debug)]
struct Society<'a> {
    name: String,
    students: Vec<&'a Student>,
}
#[derive(Debug)]
struct Course<'a> {
    name: String,
    students: Vec<&'a Student>,
}
#[derive(Debug)]
struct Student {
    name: String,
}
