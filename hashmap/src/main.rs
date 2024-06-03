use std::collections::HashMap;

fn main() {
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();

    //passing the string like this will move the ownership to the hashmap
    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    //if green is not present create a new entry but if green is present do nothing
    scores.entry(String::from("green")).or_insert(30);
    scores.entry(String::from("green")).or_insert(40);

    //search
    let team_name = String::from("blue");
    match scores.get(&team_name) {
        Some(score) => println!("{}", score),
        _ => println!("Value not found")
    };

    //print
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //map stores frequency of each word
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
