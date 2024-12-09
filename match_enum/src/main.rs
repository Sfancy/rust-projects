// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }
// fn main() {
//     let u = value_in_cents(Coin::Quarter(UsState::Alabama));
//     println!("{u}");
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Option::None => None,
//         Option::Some(i) => Some(i + 1),
//     }
// }
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

// fn main() {
//     let dice_roll = 3;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }
// }

// fn main() {
//     let dice_roll = 3;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => role(), // when we don't need the value
//     }
// }

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => move_fancy_hat(),
        _ => (), // use empty tuple type and do nothing
    }
}
