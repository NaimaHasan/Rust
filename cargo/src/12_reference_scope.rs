fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    //variables r1 and r2 will not be used after this point
    //so the scope of these references end here

    let r3 = &mut s;
    println!("{}", r3);
}
