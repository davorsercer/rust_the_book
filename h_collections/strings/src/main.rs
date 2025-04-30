#![allow(unused)]
fn main() {
    fn main() {
        let mut s1 = String::new();
    }
    let data = "initial contents";
    let s1 = data.to_string();
    println!("s1: {}", s1);

    let s2 = String::from("initial contents");
    println!("s2: {}", s2);

    let mut s3 = String::from("foo");
    s3.push_str("bar");
    println!("s3: {}", s3);

    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5);
    println!("s4: {s4}");

    let mut s6 = String::from("lo");
    s6.push('l');
    println!("s6: {}", s6);

    let s7 = String::from("Hello, ");
    let s8 = String::from("world!");
    let s9 = s7 + &s8; // note s1 has been moved here and can no longer be used
    println!("s9: {}", s9);

    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toe");
    let s13 = s10 + "-" + &s11 + "-" + &s12;
    println!("s13: {}", s13);

    let s14 = String::from("tic");
    let s15 = String::from("tac");
    let s16 = String::from("toe");

    let s17 = format!("{s14}-{s15}-{s16}");
    println!("s17: {}", s17);

    let hello = "Здравствуйте";
    let answer = &hello[0..4];
    println!("answer: {}", answer);

    let z = "З";
    println!("Bytes: {:?}", z.as_bytes());
    println!("Length: {}", z.len());

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
