// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// [macro_use](https://doc.rust-lang.org/reference/macros-by-example.html#the-macro_use-attribute)
// First, it can be used to make a module's "macro" scope not end when the module is closed
// Second, it can be used to import macros from another crate. Decorate 'extern crate' to export
// macros.
// Ex) If a macro `lazy_static` is defined inside the crate lazy_static,
// #[macro_use(lazy_static)]
// extern crate lazy_static;
// This makes lazy_static! macro inside lazy_static! crate available.

#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
