fn main() {
  let quadruple = (6, 2, -1, 0)
  println!("Tell me about {:?}", quadruple);
  // to destructure tuples, use match:
  match quadruple {
    // destructuring the two middle elements
    (6, a, b, 0) => println!("This quadruple is made of 6, {}, {} and 0", a, b),
    // `..` can be used to ignore the rest of a tuple:
    (2, ..) => println!("First comes 2, and the rest we don't care"),
    // `_` means don't bind the value to a variable
    _      => println!("It doesn't matter what they are"),
  }
}
