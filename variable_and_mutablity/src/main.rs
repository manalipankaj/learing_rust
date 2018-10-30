fn main() {
    //Immutablity
    let mut x = 5;
    println!("x is {}",x);
    x = 6;
    println!("x is {}", x);

    //Constant
    const MAX :u8 = 100;
    println!("MAX is constant {}", MAX);

    //Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("Value is x {}", y);

    //Floating-point
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    //Numberic oprations
    // addition
    let sum = 2+4; //6

    // subtrations
    let subtract = 10-4; //6

    // multiplication
    let product = 2*4; //8

    // division
    let devision = 10/5; //2

    // remainder
    let remainder = 10%3; //1
}
