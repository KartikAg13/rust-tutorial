use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x:&'a str,
    y:&'a str,  //lifetime
    ann:T   //generics
) -> &'a str
where 
    T:Display   //traits
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn main() {
    
}
