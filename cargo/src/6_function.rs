fn main() {
    println!("Hello world!");
    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    let x = five();
    let z = plus_one(5);
    println!("{y}\n{x}\n{z}");
}


fn another_function(x: i32, y: char) {
    println!("The values are: {x}{y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
