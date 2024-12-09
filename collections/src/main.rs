//fn new_vector() {
//    let _v: Vec<i32> = Vec::new();
//    // add a mutable vector
//    let mut v: Vec<i32> = Vec::new();
//    v.push(1);
//    v.push(2);
//    println!("{:?}", v);
//}

//fn new_vector_macro() {
//    let v = vec![1, 2, 3];
//    println!("{:?}", v);
//}

//fn read_vector() {
//    let v = vec![1, 2, 3, 4, 5];
//    let third = &v[2];
//    println!("The thrid element is {third}");
//
//    let third = v.get(2); // Option<&T>
//    if let Some(third) = third {
//        println!("The third elment is {third}");
//    }
//
//    let does_not_exist = &v[100]; // will panic
//    let does_not_exist = v.get(100); // will return None
//}

//fn read_vector_reference_mut() {
//    let mut v = vec![1, 2, 3, 4];
//    // immuablte borrow occurs here
//    let first = &v[0];
//    v.push(5); // cannot borrow as mutable
//    println!("The first element is: {first}");
//}

fn iterate_vector() {
    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

fn multi_type_vector_with_enum() {
    enum Spreadsheetcell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        Spreadsheetcell::Int(3),
        Spreadsheetcell::Text(String::from("blue")),
        Spreadsheetcell::Float(10.22),
    ];
}

fn main() {
    //new_vector();
    //new_vector_macro();
    //read_vector();
    //read_vector_reference_mut();
    iterate_vector();
}
