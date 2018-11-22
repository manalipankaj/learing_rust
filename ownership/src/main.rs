fn main() {
    println!("Hello, world!");
    ownership();
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
