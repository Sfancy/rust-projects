// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     let mut s = String::from("Hello from the other side");
//     let position = first_word(&s);
//     s.clear();
//     println!("{s}, {position}");
// }

// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..];
//     println!("{hello}, {world}");

//     let hello2 = "hello";
//     if hello == hello2 {
//         println!("Equal");
//     }
// }

// fn main() {
//     let s = String::from("hello from the other side");

//     let len = s.len();
//     let slice1 = &s[..2];
//     let slice1 = &s[0..2];
//     let slice2 = &s[3..len];
//     let slice2 = &s[3..];
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world again");
    let word = first_word(&s);
    println!("word: {word}")
}
