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

    //Boolean type
    let a = true;
    let b :bool = false;

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x ,y, z) = tup; //Destructuring
    println!("Values are {} {} {}", x, y, z);
    println!("Float value is {}", tup.1)

    //Arrays
    let a = [1,2,3,4,5];
    let x [i32; 5] = [1,2,3,4,5];
    let first = a[0] //Accessing elements
}
