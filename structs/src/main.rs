
struct User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool
}

//Debug is a trait
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}

impl Rectangle {
    //method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    //associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {

    //make instance of the struct
    let mut user1 = User {
        email: String::from("something@mail.in"),
        username: String::from("kartik"),
        active: true,
        sign_in_count: 1
    };

    //
    let name = user1.username;
    user1.username = String::from("ka");

    let user2 = build_user(user1.email, name);

    //create instance from some existing data
    let _user3 = User {
        email:String::from("someone@out.in"),
        username:String::from("agrawal"),
        ..user2
    };

    //tuple struct
    struct Color(User, i32, i32);
    struct Point(User, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("Rectangle: {:#?}", rect);

    println!("The area of the rectangle is: {}", rect.area());

    let rect1 = Rectangle {
        width: 40,
        height: 50
    };

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));

    //call associated function
    let square = Rectangle::square(25);
}

//field init shorthand syntax - name of the parameters of the function and struct are same, therefore no need to specify the field
fn build_user(email:String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}