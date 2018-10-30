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
}
