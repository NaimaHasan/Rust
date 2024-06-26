fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let hello = &s[..5];

    let world = &s[6..11];
    let world = &s[..11];

    let len = s.len();
    let entire = &s[0..len];
    let entire = &s[..];

    println!("{hello} {world} {entire}");

    let word = first_word(&s);

    //s.clear(); ERROR!
    //Recall from the borrowing rules
    //if we have an immutable reference to something
    //we cannot also take a mutable reference.
    //Because clear needs to truncate the String,
    //it needs to get a mutable reference

    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_literal(&my_string[0..6]);
    let word = first_word_literal(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_literal(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_literal(&my_string_literal[0..6]);
    let word = first_word_literal(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_literal(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_literal(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
