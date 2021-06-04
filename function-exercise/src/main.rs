#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
    fizzbuzz_to(100);
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 3.0),
    };

    square.translate(Point::new(2.0, 3.0));
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(number: u32) -> () {
    if is_divisible_by(number, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(number, 5) {
        println!("fizz");
    } else if is_divisible_by(number, 3) {
        println!("buzz")
    } else {
        println!("{}", number);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

#[allow(dead_code)]
impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x2 - x1) + (y2 - y1)).abs()
    }
    fn translate(&mut self, p: Point) {
        self.p1.x += p.x;
        self.p1.y += p.y;
        self.p2.x += p.x;
        self.p2.y += p.y;
    }
}

#[allow(dead_code)]
struct Pair(Box<u32>, Box<u32>);

#[allow(dead_code)]
impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}
