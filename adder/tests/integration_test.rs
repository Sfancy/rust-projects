use adder::add;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    let result = add(2, 4);
    assert_eq!(result, 6);
}
