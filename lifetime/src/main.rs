struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement:&str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {

    /*
        1. Each parameter that is a reference gets its own lifetime parameter
        2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
        3. If there are multiple input lifetime parameters, but one of them is &self or &mut self the lifetime of self is assigned to all output lifetime parameters
    */

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    //here, the lifetime of result is same as string1 and string2 since both have same lifetime
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find word");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("{}", i.part);

    //static lifetime
    //reference lives till the end of the program
    let _s:&'static str = "I have a static lifetime"; 
}

//generic lifetime annotations
//used to create relationships between different lifetimes 
//here, the lifetime of return value will be same as of the variable with shortest lifetime
fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}