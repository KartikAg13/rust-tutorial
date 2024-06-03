fn main() {

    //vector
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    //alternate way
    let mut v2 = vec![1, 2, 3, 4, 5];

    //general way
    let third = &v2[2];
    println!("The third element is {}", third);

    //error handling
    match v2.get(2) {
        Some(number) => println!("The element is {}", number),
        None => println!("There is no third element")
    };
    
    //iterating 
    for i in &mut v2 {
        *i += 50;   //dereferencing operator
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer")
    };
}
