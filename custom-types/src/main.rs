use crate::List::{Cons, Nil};
use std::fmt;

#[derive(Debug)]
struct Pair(i32, f32);

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "top left: {}, bottom right: {}",
            self.top_left, self.bottom_right
        )
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn area_rectangle(rect: Rectangle) -> f32 {
    let point_top_left = rect.top_left;
    let point_bottom_right = rect.bottom_right;
    let width = (point_bottom_right.x - point_top_left.x).abs();
    let height = (point_top_left.y - point_bottom_right.y).abs();
    return width * height;
}

fn square(point: Point, unit: f32) -> Rectangle {
    let length = point.x + unit;
    let height = point.y + unit;
    let top_left = Point {
        x: point.x,
        y: height,
    };
    let bottom_right = Point {
        x: length,
        y: point.y,
    };
    return Rectangle {
        top_left,
        bottom_right,
    };
}

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{} {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    println!("Welcome to the world of structs");
    let test_point = Point { x: 10.0, y: 20.0 };
    let test_second_point = Point { x: 30.0, y: 40.0 };
    println!(
        "Area : {}",
        area_rectangle(Rectangle {
            top_left: test_point,
            bottom_right: test_second_point
        })
    );
    println!("Rectangle: {}", square(test_point, 22.0));

    //create an empty list
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("{}", list.stringify());
    println!("Length of the final list: {}", list.len());
}
