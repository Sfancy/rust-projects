// fn main() {
//     let s1 = String::from("Hello");
//     let len = caculate_length(&s1);
//     println!("The length of '{s1}' is {len}");
// }
//
// fn caculate_length(s: &str) -> usize {
//     s.len()
// }

// Mutable reference
// fn main() {
//     let mut s = String::from("Hello");
//     change(&mut s);
//     println!("S is {s}");
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// can not have 2 mutable reference at a time
// fn main() {
//     let mut s = String::from("Hello");
//
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{}, {}", r1, r2);
// }

fn main() {
    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}
