fn main(){
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing)
}

//fn dangle() -> &String { // dangle returns a reference to a String
    //let s = String::from("hello");
    //&s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

//ERROR: this function's return type contains a borrowed value, 
//but there is no value for it to be borrowed from

//Because s is created inside dangle, 
//when the code of dangle is finished, s will be deallocated. 
//But we tried to return a reference to it. 
//That means this reference would be pointing to an invalid String


fn no_dangle() -> String {
    let s = String::from("hello");
    s
}