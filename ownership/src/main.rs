fn main() {
    
    /*
        1. each value in rust has a variable that's called its owner
        2. there can only be one owner at a time
        3. when the owner goes out of scope, the value will be dropped
        4. you can only have one mutable reference to a particular piece of data in a particular scope
        5. you cannot have a mutable reference if a immutable reference exists
    */

    let mut s1 = String::from("hello");
    let s2 = s1;    //moves the value from s1 to s2
    let mut s3 = s2.clone();    //copies the value in s2 to s3


    //give and take ownership from function
    s1 = takes_and_gives_back(s2);
    
    //s2 does not exist now

    //passing references is known as borrowing
    //references are also immutable by default
    let len = calculate_length(&mut s1);
    println!("The length of {} is {}", s1, len);

    //references
    let _r1 = &s3;
    let _r2 = &s3;
    
    //scope of r1 and r2 ends here because they are not used afterwards

    //mutable reference can be made here because the immutable references r1 and r2 do not exist
    let r3 = &mut s3; 

    let word = first_word(&r3);
    println!("The first word is {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn takes_and_gives_back(a_string:String) -> String {
    a_string
}

fn calculate_length(s:&mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn first_word(s:&String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; //string slices
        }
    }
    &s[..]  //returns the entire string
}