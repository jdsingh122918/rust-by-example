use std::fmt;

// Struct holding two number
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement Display trait for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Struct to store 2D
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Implement Display trait for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Struct to store complex numbers
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// Implement Display trail for complex numbers
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// Struct to store a list of vectors
#[derive(Debug)]
struct List(Vec<&'static str>);

// Implement Display trait for list of vectors
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}:{}", count, v)?;
        }
        write!(f, "]")
    }
}

// Struct to store city along with latitude and longitude
#[derive(Debug)]
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

// Implement Display trait for the City
impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}: {:.3}°{} {:3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:02x}{:02X}{:02X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lon: -6.259722,
            lat: 53.347778,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", *color);
    }
    let minmax = List(vec!["hello", "world", "jodhan"]);

    println!("Comparing structures");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
}
