use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    //panic
    a();

    //result enum
    let f = File::open("hello.txt");
    
    //unwrap() returns file is successful else panics
    let f = File::open("hello.txt").unwrap();

    //expect() is used to specify the panic message 
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    //handle the error using closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        }
        else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    //our main() can return result, when success returns nothing but in case of error...
    //this enables us to use '?' in the main()
    let f = File::open("hello.txt")?;
    //when everything succeds we returns nothing
    Ok(())
}

fn a() {
    b();
}

fn b() {
    c(21);
}

fn c(num:i32) {
    if num == 22 {
        //use "RUST_BACKTRACE=1 cargo run" to trace the error
        panic!("Don't pass in 22!");
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    //? is used to check if there is an error or not and it can only be used in functions that return result or option enum
    let mut s = String::new(); 
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}