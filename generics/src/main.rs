struct Point<T> {
    x:T,
    y:T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    
    //create a generic function
    fn mix<U>(self, other:Point<U>) -> Point<U> {
        Point {
            x: other.x,
            y: other.y
        }
    }
}

//can only be called when the types are f64
impl Point<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest(char_list);
    println!("The largest character is {}", largest);

    //generics in structs
    let p1 = Point { x: 5, y: 10};
    let p2 = Point{ x: 0.5, y: 0.10 };
    let answer = p2.x() + p2.y();
    println!("The answer is {}", answer);

    //generics are used in option and result enum
    /*
    enum Option<T> {                    enum Result<T, E> {
        Some(T),                            Ok(T),
        None                                Err(E)
    }                                   }
    */
    
    let p3 = Point { x: 'a', y: 'b' };
    let p2 = p1.mix(p3);
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
    
}

//generic function with a trait restricting its possible types, trait bound
fn get_largest<T:PartialOrd + Copy>(list:Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}