use std::collections::HashMap;

#[allow(unused)]
fn new_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
}

#[allow(unused)]
fn access_value_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // get return Option<&V> or None if key not exist
    // copied return get actual value instead of reference
    // unwrap_or set to zero when it's None
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    let score = scores.get("Yellow").copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

#[allow(unused)]
fn update_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 40); // will replace
    println!("{scores:?}");

    // The return value of the entry method is an enum
    // called Entry that represents a value that might or might not exist.
    scores.entry(String::from("Blue")).or_insert(60); // will not change

    // The or_insert method on Entry is defined to return a mutable reference
    // to the value for the corresponding Entry key if that key exists, and if not,
    // it inserts the parameter as the new value for this key and returns a mutable
    // reference to the new value
    scores.entry(String::from("Yellow")).or_insert(60); // will insert
    println!("{scores:?}");
}

fn update_base_old() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert return a mutable reference
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}

#[allow(unused)]
fn try_object() {
    #[derive(Debug)]
    enum Values {
        Text(String),
        Number(i32),
    }
    let mut object = HashMap::new();
    object.insert(String::from("name"), Values::Text(String::from("John Doe")));
    object.insert(String::from("age"), Values::Number(23));

    println!("{:?}", object);

    for (key, value) in &object {
        if let Values::Text(name) = value {
            println!("{key}: {name}");
        }
        if let Values::Number(age) = value {
            if key == "age" {
                println!("Get age: {age}");
            }
        }
    }
}

fn main() {
    //new_hash_map();
    //access_value_hash_map();
    //try_object();
    //update_hash_map();
    update_base_old();
}
