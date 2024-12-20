// Box is available directly in std namespace
// but Rc is not, so we need to import it
use std::{rc::Rc, vec};

struct Rectangle {
    height: u32,
    width: u32,
}

// it is idiomatic to use impl block for functions operating on Rectangle,
// unless something specific is needed and we need to implement functions
// outside the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn set(&mut self, height: u32, width: u32) {
        self.height = height;
        self.width = width;
    }
}

// Structure representing a student.
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
}

// Increasing the age of a student
impl Student {
    // This implementation is wrong. There is a possibility of overflow.
    // Even if we decide age is u128. The same concern/overflow risk exists.
    fn increase_age(&mut self, years: u8) {
        self.age += years;
    }
}

// Increasing the age of a student must be implemented outside the struct, and use Option.
fn increase_age(student: Option<&mut Student>, years: u8) -> Option<&mut Student> {
    // the idiomatic way
    student.and_then(|s| {
        s.age.checked_add(years).map(|new_age| {
            s.age = new_age;
            s
        })
    })

    /*
    // this works, but it is less idiomatic because it is more verbose
    // and requires additional variable, that the above code does within a closure
    let s = student?;
    s.age.checked_add(years).map(|new_age| {
        s.age = new_age;
        s
    })

    // this also works, but it is even more verbose
    // and requires a manual wraping
    // the unwrapped value back to Option
    let s = student?;
    s.age = s.age.checked_add(years)?;
    Some(s)
    */
}

// Structure representing a point in 2D space.
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = (self.x - other.x).pow(2);
        let y_squared = (self.y - other.y).pow(2);
        // calculate the square root of the sum of the squares of x and y
        f64::sqrt(x_squared as f64 + y_squared as f64)
    }
}
// distances between points with the use of for loop
fn demo_points_wrong() {
    let v = [Point::new(0, 0), Point::new(1, 1), Point::new(2, 2)];
    let mut distances = Vec::new();
    // this line is super error-prone and hard to read
    // NOTE: in Rust, the range notation takes first inclusive and second exclusive!  Something to MEMORISE!
    for i in 0..v.len() - 1 {
        distances.push(v[i].distance(&v[i + 1]));
    }
    println!("Distances for our points: {:?}", distances);
}

fn demo_points() {
    let v = [Point::new(0, 0), Point::new(1, 1), Point::new(2, 2)];

    let distances = v
        .windows(2)
        .map(|w| w[0].distance(&w[1]))
        .collect::<Vec<f64>>();
    /*
    * Coming soon in newer Rust releases
    * ability to infer the window from the closure later in the pipeline

      let d2 = v
          .array_windows()
          .map(|left, right| left.distance(right))
          .collect::<Vec<f64>>();

    */

    println!("Distances for our points: {:?}", distances);
}

fn demo_move(p: Point) -> Point {
    println!("Point: ({}, {})", p.x, p.y);
    p
}

fn demo_borrow(p: &Point, r: &Point) {
    println!("Point: ({}, {})", p.x, p.y);
    println!("Point: ({}, {})", r.x, r.y);
}

fn demo_move_int(i: i32) -> i32 {
    i
}

fn use_demo_move() {
    let i = 10;
    demo_move_int(i);
    // THIS IS co-pilot generated comment: i is moved, so we cannot use it anymore
    // and it is WRONG! i is not moved, it is copied
    println!("i: {}", i);

    let p = Point::new(1, 2);
    let p2 = demo_move(p);
    // p is moved, so we cannot use it anymore
    println!("Point: ({}, {})", p2.x, p2.y);

    demo_borrow(&p2, &Point::new(3, 4));
    print!("Point: ({}, {})", p2.x, p2.y);

    let p_mover = || p2;

    let p3 = p_mover();

    println!("Point: ({}, {})", p3.x, p3.y);
}

fn main() {
    demo_points();
    demo_points_wrong();

    // similar to C++ concept of: unique_ptr<Rectangle>
    let rect = Box::new(Rectangle {
        height: 10,
        width: 20,
    });
    // similar to C++ shared_ptr<Rectangle>
    // compare these two lines
    // let mut rect2 = Rc::new(rect);
    let mut rect2 = Rc::new(*rect);

    // lets mutate our rect2
    Rc::get_mut(&mut rect2).unwrap().set(20, 30);
    println!("rect2 area: {}", rect2.area());

    // rect does not own the value anymore, it has been moved
    // compiler error:
    // println!("rect area: {}", rect.area());

    // on the stack
    let rect3 = &mut Rectangle {
        height: 10,
        width: 20,
    };
    rect3.set(20, 30);
    println!("rect3 area: {}", rect3.area());

    // playing with vectors
    let v = [1, 2];
    {
        let first = &v[0]; // this will panic if index is out of bounds
        let second = v.get(1); // returns Option to a reference
        let second_value = second.unwrap_or(&0);

        // first = &v[4];
        println!("first: {}, second: {}", first, second_value);
    }

    let mut v = vec![1, 2, 3];
    match v.get(4) {
        Some(value) => println!("v[4]: {}", value),
        None => println!("v[4] is out of bounds"),
    };
    if let Some(x) = v.get(4) {
        println!("v[4]: {}", x);
    } else {
        println!("v[4] is out of bounds");
    }

    let x = &mut v[0];
    // Why is this frawn upon in Rust? And what Clippy suggests you should use instead?
    *x = *x * 10;
    println!("New values in our vector v: {:?}", v);

    let mut v = vec![1, 2, 3];
    if let Some(x) = v.get(3) {
        println!("New value times 10: {}", x * 10);
    }

    v.get(4).map_or_else(
        || println!("v[4] is out of bounds"),
        |value| println!("v[4]: {}", value),
    );

    // first and second are out of scope and cleaned
    v.push(4);
    println!("v[0]: {}", &v[0]);

    let mut stud = Student {
        name: String::from("Mariusz"),
        age: 21,
    };
    println!("Student: {:?}", increase_age(Some(&mut stud), 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_area() {
        let rect = Rectangle {
            height: 10,
            width: 20,
        };
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn student_age_normal() {
        let mut stud = Student {
            name: String::from("Mariusz"),
            age: 21,
        };
        stud.increase_age(21);
        assert_eq!(stud.age, 21 + 21);
    }

    #[test]
    fn student_age_overflow() {
        let mut stud = Student {
            name: String::from("Mariusz"),
            age: 21,
        };
        let stud = increase_age(Some(&mut stud), 255);
        assert_eq!(stud.unwrap().age as u16, 21 + 255);
    }
}
