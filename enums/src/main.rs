use std::i8;

enum IpAddrKind {
    V4,
    V6,
}

//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}
//fn main() {
//    let four = IpAddrKind::V4;
//    let six = IpAddrKind::V6;
//    let home = IpAddr {
//        kind: IpAddrKind::V4,
//        address: "127.0.0.1".to_string(),
//    };
//    let loopback = IpAddr {
//        kind: IpAddrKind::V6,
//        address: String::from("::1"),
//    };
//}

//enum IpAddr {
//    V4(u8, u8, u8, u8),
//    V6(String),
//}
//
//fn main() {
//    let home = IpAddr::V4(127, 0, 0, 1);
//    let loopback = IpAddr::V6(String::from("::1"));
//}

//enum Message {
//    Quit,                    // No data associated, like unit struct
//    Move { x: i32, y: i32 }, // Named fields like struct
//    Write(String),
//    ChangeColor(i32, i32, i32),
//}
//impl Message {
//    fn call(&self) -> &str {
//        "called"
//    }
//}
//fn main() {
//    let m = Message::Quit;
//    println!("{}", m.call());
//}

//fn main() {
//    let some_number = Some(5); // can infer it to be Option<i32>
//    let some_char = Some('E'); // can infer to Option<char>
//    let absent_number: Option<i32> = None; // cannot be inferred
//                                           //Option::None is equal to None
//}

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
