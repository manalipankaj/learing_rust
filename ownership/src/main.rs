fn main() {
    println!("Hello, world!");
    ownership();
    let len = calculate_length(String::from("Hello"));
    println!("lenght of {} is {}",len.0, len.1);

    let s1 = String::from("Hello");
    let len2 = calculate_length_passing_ref( &s1 );
    println!("length of {} is {}", s1, len2 );

    let mut newStr = String::from("Hello");
    // let r1 = &mut newStr;  //Will get error at next borrow statement, you cannot borrow refference more than once
    // let r2 = &newStr;    //Error, you cannot have mutable refference when you have immutable refference

    change(&mut newStr);
    // change2(&mut newStr);
}

fn ownership() {
    {
        //Scope of s
        let mut s = String::from("hello");

        s.push_str(", world"); //appends to a string

        println!("String is {}", s);
    }   // s is no longer valid

    // move variable
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1 is {}", s1); //Will give error becuase s1 is now valid anymore

    // clone data
    let x = String::from("hello");
    let y = x.clone();
    println!("x is {} and y is {}", x, y);

    // Why move doesn't work here
    let a = 5;
    let b = a;
    println!("a is {} and b is {}", a, b);  // This works because interger is stored in stack memory
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
fn calculate_length_passing_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}
