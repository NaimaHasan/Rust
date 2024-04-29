use std::io;

fn main() {
    //The variable tup binds to the entire tuple because a tuple is considered a single compound element.
    let tup: (i32, f64, u8) = (500, 6.4, 1); //type annotation optional
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y = {y}");

    let t1 = tup.0;
    let t2 = tup.1;
    let t3 = tup.2;
    println!("({t1}, {t2}, {t3})");

    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a = [3; 5]; //[value; length]
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //[type; length]


    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}


