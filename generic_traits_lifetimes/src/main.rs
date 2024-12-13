/* Duplicate largest function */
/*

// list[0] creates an owned value of type i32, so it need use with "&"
// for item in list will create a reference automatically
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);

    println!("The largest number is {result}");

    let char_list = vec!['a', 'b', 'c', 'd'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}
*/

// /* using generic without restriction */
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {result}");
// }
//
// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }
//
// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
//
// fn main() {
//     let point1 = Point { x: 5, y: 10.4 };
//     let point2 = Point { x: "hello", y: 'c' };
//     let point3 = point1.mixup(point2);
//     println!("point3.x = {}, point3.y = {}", point3.x, point3.y);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
//
// fn main() {
//     let s1 = "abdc";
//     let s2 = "efg";
//     let s3 = longest(s1, s2);
//     println!("longest string is {s3}");
// }

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    longest_with_an_announcement("x", "y", "");
}
