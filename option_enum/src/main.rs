fn main() {
    let some_number = Some(5);
    let _some_string = Some("a string");
    
    //it is important to specify the type when no value is passed
    let _absent:Option<i32> = None;

    //add i32 to Option<i32>
    let x = 5;
    let _sum = x + some_number.unwrap_or(0);

    //add 1 to none
    let _none = plus_one(None);
    let _six = plus_one(Some(5));

    //if let syntax
    if let Some(5) = some_number {
        println!("five");
    }
}

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None   //if x does not match the above statement(s) then return None
    }
}