fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.",s1,len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    let mut s = String::from("hello");

    //let r1 = &mut s;
    //let r2 = &mut s;
    //println!("{}, {}", r1, r2);
    //Error: first borrow later used


    let r1 = &s; // no problem
    let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.


fn change(s: &mut String){
    s.push_str(", world");
}