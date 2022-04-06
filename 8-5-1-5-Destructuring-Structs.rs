fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
        z: [u32; 3],
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (2, 2), y: 2, z: [6, 4, 3] };

    match foo {
        Foo { x: (1, b), y, z } => println!("First of x is 1, b = {},  y = {} and z = {:?} ", b, y, z),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i, z: j } => println!("y is 2, i = {:?} and my array = {:?}", i, j),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
}
