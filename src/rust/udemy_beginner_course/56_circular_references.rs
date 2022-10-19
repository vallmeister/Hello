#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

// objects that reference each other
// students* --- *course

// normalization:
// students
// course
// Vec<Enrollment { course, student }>

/* Doesn't work:
struct Student<'a> {
    name: String,
    courses: Vec<&'a Course<'a>>
}

impl<'a> Student<'a> {
    fn new(name: &str) -> Student<'a> {
        Student {
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course<'a> {
    name: String,
    students: Vec<&'a Student<'a>>
}

impl<'a> Course<'a> {
    fn new(name: &str) -> Course<'a> {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(&'a mut self, student: &'a mut Student<'a>) {
        student.courses.push(self);
        self.students.push(student);
        // RefCell
    }
}*/

struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }
    fn add_student(
        course: Rc<RefCell<Course>>,
        student: Rc<RefCell<Student>>
    ) {
        student.borrow_mut().courses.push(course.clone()); // We need to clone courses because we can't borrow it mutable and immutable once each
        course.borrow_mut().students.push(student.clone());
        course.borrow_mut().students.push(student);
    }
}

// normalized approach
struct Student2 {
    name: String
}

impl Student2 {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Course2 {
    name: String
}

struct Enrollment<'a> {
    student: &'a Student2,
    course: &'a Course2
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student2, course: &'a Course2) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self,
        student: &'a Student2,
        course: &'a Course2) {
            self.enrollments.push(
                Enrollment::new(student, course)
            );
    }
}

fn main() {/*
    let john = Student::new("John");
    let course = Course::new("Rust Course");

    // Imagine drops: If we drop course first, John stil has a reference that's being dropped.
    // If we drop John first, the course will have a reference to an inexistent person
    course.add_student(john); //Rc*/

    let john = Rc::new(
        RefCell::new(
            Student::new("John")
        )
    );
    let jane = Rc::new(
        RefCell::new(
            Student::new("Jane")
        )
    );
    let course = Course::new("Rust Course");
    let magic_course = Rc::new(RefCell::new(course));

    Course::add_student(magic_course.clone(), john);
    Course::add_student(magic_course, jane);

    let course2 = Course2 { name: "Intro to Rust".into() };
    let john2 = Student2 { name: "John". into() };

    let mut p = Platform::new();
    p.enroll(&john2, &course2);

    for c in john2.courses(p) {
        println!("John is taking {}", c);
    }
    // Takeway: Circular references are possible with RC and RefCell,
    // but should be avoided at all by redesigning the code. RC and RefCell
    // lose out on all the borrow-checker stuff from the compiler.
}
