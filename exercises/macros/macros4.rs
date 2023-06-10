// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

// `;` separates each macro arm.
// macro automatically matches its input to each arm into following arm body.
// So `;` is enough to split each arm.
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
