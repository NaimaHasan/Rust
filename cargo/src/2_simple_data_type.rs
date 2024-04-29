fn main() {
    let x = 2.7; // f64
    let y: f32 = 3.0; // f32
    println!("{x}, {y}\n");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("sum = {sum}\ndifference = {difference}");
    println!("product = {product}\nquotient = {quotient}");
    println!("truncated = {truncated}\nremainder = {remainder}\n");


    let t = true;
    let f: bool = false; //with explicit type annotation
    println!("{t}, {f}\n");


    let c = 'z';
    let z: char = 'ℤ'; //with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}");
}
