fn main(){
    let s = "hello"; //immutable, fixed length string literal
    println!("{}",s);

    let s = String::from("hello"); //string type, able to store an amount of text that is unknown to us at compile time. 
    println!("{}",s);
    
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);
    //Won't work bc to ensure memory safety rust considers s1 to be no longer valid.
    //To prevent double free error.

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    //No issue here because clone does deep copy. 
    //Copies both the stck and the heap.



    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
