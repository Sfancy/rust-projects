//fn main() {
//    let config_max = Some(3u8);
//
//    match config_max {
//        Some(max) => println!("The maximum is configured to be {max}"),
//        _ => (),
//    }
//}

//fn main() {
//    let config_max = Some(3u8);
//    if let Some(max) = config_max {
//        println!("The maximum is configured to be {max}");
//    } else {
//        println!("Do something else.");
//    }
//}

// --------
enum Coin {
    Quarter(String),
    Nickel(String),
    Dime,
}

fn main() {
    let coin = Coin::Quarter(String::from("A big coin"));
    if let Coin::Quarter(desc) = coin {
        println!("{}", desc);
    } else if let Coin::Nickel(desc) = coin {
        println!("{}", desc);
    } else {
        println!("must be Dime");
    }
}
