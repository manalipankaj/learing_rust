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
    }
}
