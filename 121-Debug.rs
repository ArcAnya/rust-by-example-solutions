// need to add this line to be able to manually format with Display here
use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// Question: what's this <'_> for? 
impl Display for Person<'_> {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        write!(f, "His name is {} and he is {} years old.", self.name, self.age)
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{}", peter);
}
