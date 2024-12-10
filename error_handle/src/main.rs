use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

#[allow(unused)]
fn create_panic() {
    panic!("crash and burn");
}

#[allow(unused)]
fn out_of_index() -> i32 {
    let v = vec![1, 2, 3];
    v[99]
}

#[allow(unused)]
fn open_unexist_file() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

#[allow(unused)]
fn use_unwrap() {
    // let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

#[allow(unused)]
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

#[allow(unused)]
fn short_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(unused)]
fn even_shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(unused)]
fn ultimate_short_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

#[allow(unused)]
fn last_char_of_fist_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    use_unwrap();
    // open_unexist_file();
    //out_of_index();
    //create_panic();
}
