use unicode_segmentation::UnicodeSegmentation;

fn main() {
    //Strings are stored as a collection of UTF-8 encoded bytes
    let _s1 = String::new();
    let s2 = "initial comments!";
    let _s3 = s2.to_string();
    let mut s4 = String::from("initial");

    s4.push_str(" comments");
    s4.push('!');

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");

    //add two strings
    //without taking ownership
    let _s7 = format!("{}{}", s5, s6);

    //s5 is moved so the value does not exist
    let _s7 = s5 + &s6;

    let s = String::from("hello");

    //print bytes
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();

    //print scalar values
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();

    //print grapheme clusters
    for g in s.graphemes(true) {
        print!("{} ", g);
    }
    println!();
}
