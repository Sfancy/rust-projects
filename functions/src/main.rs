// fn five() -> u8 {
//     255
// }

// fn main() {
//     // println!("Hello, world!");

//     // print_labeled_measurement(10, 'm');
//     let x = five();
//     println!("The value of {x}");
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is : {value} {unit_label}")
// }

fn main() {
    let x = plus_one(5).unwrap();

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> Option<i32> {
    Some(x + 1);
    None
}
