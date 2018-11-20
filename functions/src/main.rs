fn main() {
    another_function(100);
    other_function(200, 300);
    let add = add_function(100,200);
    print_function(add);
}

//Function with one parameter
fn another_function(x: u32) {
    println!("X is {}",x);
}

//Function with two parameters
fn other_function (x: u32, y: i32){
    println!("x is {} and y is {}", x, y);
}

//Function with return Value
fn add_function (x: i32, y: i32) -> i32 {
    x + y
}

fn print_function (x: i32) {
    println!("You want to print : {}",x);
}
