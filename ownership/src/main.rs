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
    println!("s1 is {}", s1);
}
