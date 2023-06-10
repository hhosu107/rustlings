// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// Macros must be pre-defined before its usage; unlike to functions.
fn main() {
    my_macro!();
}
