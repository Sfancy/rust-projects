//struct User {
//    active: bool,
//    username: String,
//    email: String,
//    sign_in_count: u64,
//}
//
//fn main() {
//    let user1 = User {
//        active: true,
//        username: String::from("someusername123"),
//        email: String::from("someone@example.com"),
//        sign_in_count: 1,
//    };
//    println!("{}", user1.username);
//}

//fn main() {
//    let mut user2 = User {
//        active: true,
//        username: String::from("someone"),
//        email: String::from("someone@example.com"),
//        sign_in_count: 1,
//    };
//    user2.email = String::from("anotheremail@exaple.com");
//
//    println!("{}", user2.email);
//}

// * Field shorthand
//fn build_user(email: String, username: String) -> User {
//    User {
//        active: true,
//        username,
//        email,
//        sign_in_count: 1,
//    }
//}

// * create instance from other instance
//fn main() {
//    let user = build_user("someone".to_string(), "someemail@example.com".to_string());
//
//    let user2 = User {
//        active: user.active,
//        username: user.username,
//        email: String::from("another@example.com"),
//        sign_in_count: user.sign_in_count,
//    };
//    // println!("{}, {}", user.username, user2.username);
//
//    let user3 = User {
//        email: String::from("yetanother@example.com"),
//        ..user2
//    };
//}

// * tuple struct
//struct Color(i32, i32, i32);
//struct Point(i32, i32, i32);
//fn main() {
//    let black = Color(0, 1, 2);
//    let origin = Point(1, 2, 3);
//    println!("{}, {}", black.1, origin.0);
//}

// * tuple struct
#[derive(Debug, PartialEq, Eq)]
struct AlwaysEqual;
fn main() {
    let subject1 = AlwaysEqual;
    let subject2 = AlwaysEqual;

    assert_eq!(subject1, subject2);
    println!("Subjects are equal!");
}
