//fn update_a_string() {
//    let mut s = String::from("foo");
//    s.push_str("bar");
//    println!("{s}");
//}

#[allow(unused)]
fn plus_operator() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let s3 = s1 + &s2;
    println!("{}", s3); // note s1 has been moved here and can no longer be used
}

#[allow(unused)]
fn use_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

#[allow(unused)]
fn slice_string() {
    let hello = "Здравствуйте"; // take 24 bytes, 2 bytes as a char
    let s1 = &hello[0..4]; // will return "Зд"
    let s2 = &hello[0..1]; // will panic; byte index 1 is not a char boundary
    println!("{s1},{s2}");
}

fn chars() {
    for c in "Здравствуйте".chars() {
        println!("{c}");
    }
}

fn main() {
    //update_a_string();
    //plus_operator();
    //use_format();
    //slice_string();
    chars();
}
