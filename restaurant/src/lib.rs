pub mod another_module {
    pub struct SomeStructInCurrentFile;
}

// Note we need to declare it as a module;
// it will look for front_of_house.rs and front_of_house/mod.rs
// put all content in that file as the module content
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

/*
 Why Rust Requires Module Declarations
Explicitness in Code Structure

By explicitly declaring modules in your code (e.g., mod my_module;), you signal which files are part of the module system. This avoids accidental inclusion of unrelated or unintended files in your project, making the codebase easier to reason about.
Flexibility in Module Organization

Rust allows you to define modules in different ways:
Inline within the same file.
In a separate file.
In subdirectories (using mod.rs or the newer convention of just naming the file after the module).
By requiring explicit declarations, you have control over how and where modules are included, which can be crucial for larger projects.
Avoiding Magic Behavior

Automatically treating every file as a module would be considered "magic" and could lead to unintended side effects or dependencies. Rust’s philosophy prefers you to state your intentions clearly rather than having the compiler infer them implicitly.
Clearer Dependencies

Explicit module declarations make it easier to track the dependencies between different parts of the codebase. This can help in debugging and refactoring.
 */
