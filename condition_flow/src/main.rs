fn main() {
    println!("Hello, world!");
    if_if();
    let_if();
    loops();
    whiles();
    for_loop();
}

fn if_if () {
    let number = 3;
    if number > 5 {
        println!("Number is greater");
    }
    else {
        println!("Number is smaller");
    }
}

fn let_if() {
    let number = 5;

    let x = if number > 5 {
        number-1
    } else {
        number+1
    };

    println!("The number is {}", x);
}

fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 100 {
            break counter;
        }
    };

    assert_eq!(result, 100);
}

fn  whiles() {
    let mut x = 0;

    while x < 10 {
        println!("hello {}",x);
        x += 1;
    }
}

fn for_loop() {
    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("element is {}", element);
    }

    for number in (1..10) {
        println!("number is {}", number)
    }

    for number in (1..10).rev() {
        println!("number is {}", number)
    }
}
