fn main() {
    println!("Hello, world!");
    ownership();
    let len = calculate_length(String::from("Hello"));
    println!("lenght of {} is {}",len.0, len.1);

    let s1 = String::from("Hello");
    let len2 = calculate_length_passing_ref( &s1 );
    println!("length of {} is {}", s1, len2 );
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
