fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {result}");
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// The function longest_with_an_announcement takes two string slices and a value of any type that implements the Display trait.
// It prints an announcement and returns the longest of the two string slices.
// The lifetime 'a indicates that the returned string slice will live at least as long as the shorter of the two input string slices.
