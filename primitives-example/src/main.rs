use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[allow(unused)]
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (a, b, c, d) = (self.0, self.1, self.2, self.3);
        writeln!(f, "({} {})", a, b);
        writeln!(f, "({} {})", c, d)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let (a, b, c, d) = (matrix.0, matrix.1, matrix.2, matrix.3);
    return Matrix(a, c, b, d);
}

fn main() {
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("{}", matrix);
    println!("{}", transpose(matrix));
    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("Reversed pair is {:?}", reverse(pair));
}
