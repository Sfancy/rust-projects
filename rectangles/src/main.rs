//fn main() {
//    let width1 = 30;
//    let height1 = 50;
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(width1, height1)
//    );
//}
//
//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

// * using tuple
//fn main() {
//    let rect1 = (30, 50);
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(rect1)
//    );
//}
//
//fn area(dimension: (u32, u32)) -> u32 {
//    dimension.0 * dimension.1
//}

// * struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//fn main() {
//    let rect = Rectangle {
//        width: 30,
//        height: 50,
//    };
//    println!("The rectangle is {rect:#?}");
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(&rect)
//    );
//}
//
//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}

//fn main() {
//    let rect = Rectangle {
//        width: dbg!(30 * 2),
//        height: 50,
//    };
//    dbg!(&rect);
//}

impl Rectangle {
    // &self is equal to self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area is {}", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonezero width: {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("The rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(30);
    println!("Square: {:#?}", square);
}
