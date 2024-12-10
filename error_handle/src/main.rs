use std::{arch::asm, fs::File, intrinsics::mir::Goto, io::ErrorKind};

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
    let greeting_file = File::open("hello.txt").unwrap();
}

fn main() {
    use_unwrap();
    // open_unexist_file();
    //out_of_index();
    //create_panic();
}
