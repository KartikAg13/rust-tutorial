fn main() {

    //variables are immutable by default
    let x = 5;
    println!("The value of x is {}", x);

    //shadowing
    let x = "six";
    println!("The value of x is {}", x);

    //constants are immutable
    const SUMMER_BREAK:u32 = 63_000;
    println!("Constant is {}", SUMMER_BREAK);

    //tuples
    let tup = ("Let's Get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    println!("{} has {} subscribers", channel, sub_count);

    //arrays
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    println!("Error {} Not Found", not_found);

    //declare an array [value; length]
    //stores 0 in all the 8 places
    let byte = [0; 8];
    for value in byte.iter() {
        println!("{}", value);
    }

    //function call
    let sum = my_function(11, 22);

    //control flow
    if sum < 10 {
        println!("sum is less than 10");
    }
    else if sum < 20 {
        println!("sum is less than 20");
    }
    else {
        println!("sum is greater than 20");
    }

    //similar to ternary operator
    let x = false;
    let sum = if x {sum} else {sum + 1};
    println!("The sum is {}", sum);

    //return value from loop
    let mut x = 0;
    let result = loop {
        x += 1;
        if x == 10 {
            break x;
        }
    };
    println!("The result is {}", result);

    //while loop
    let mut x = 3;
    while x != 0 {
        println!("{}!", x);
        x -= 1;
    }
    println!("LiftOff!!!");

    //range in for in loop
    for number in x..sum {
        print!("{}\t", number);
    }
}

//defination of a function
fn my_function(x:i32, y:i32) -> i32 {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    //return the sum
    x + y
}