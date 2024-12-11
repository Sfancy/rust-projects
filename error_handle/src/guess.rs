pub struct Guess {
    value: i32, // a private field, no other way to create instance
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Guess { value }
    }

    // public getter
    pub fn value(&self) -> i32 {
        self.value
    }
}
