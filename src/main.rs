/*
This is a block comment. Use this if the comments
span multiple lines.
 */
fn main() {
    // One can also use this to comment in single line
    println!("Hello, world!");
    println!("My name is JD Singh");

    // In general `{}` will be replaced by any arguments
    println!("The month of {0} has {1} days", "January", 31);

    // We can also use named arguments
    println!(
        "{subject} {object} {verb}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // special formatting after `:`
    println!("{} of {:b} people know binary, other half doesn't", 1, 2);

    use std::fmt;
    #[derive(Debug)]
    struct Structure(i32);

    // implement fmt::Display trait for struct Structure
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    println!("The value of the struct {:#?}", Structure(3));

    // Use of formatting to restrict the value to 3 decimal places
    println!("The value of Pi is {:.*}", 3, std::f64::consts::PI);
}
