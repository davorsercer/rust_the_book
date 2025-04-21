// This program demonstrates the concept of ownership and borrowing in Rust.
// It defines a function `first_word` that takes a reference to a String and returns the index of the first space character.
// The program also shows how the ownership of a String can be transferred and how it affects the validity of references to its content.
// The `first_word` function takes a reference to a String and returns the index of the first space character.
// If there is no space, it returns the length of the String.
// The main function creates a String, calls `first_word`, and then clears the String.
// The program demonstrates that after clearing the String, the index returned by `first_word` is no longer valid.
// The program will not compile if we try to use the invalid index after clearing the String.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("item: {item}, i: {i}");
        if item == b' ' {
            return &s[0..i];
            // return i;
        }
    }

    s
}

fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    // s.clear(); // this empties the String, making it equal to ""
    // now when we have string and not number, we dont have to worry about
    // the value of word, because we are not using it as an index

    println!("{word}");
    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!
}