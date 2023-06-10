// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    /*
    for x in option {
        res += x;
    }
    */
    /*
    while let Some(x) = option { // this is used to check pattern in a loop.
        res += x;
    }
    */

    if let Some(x) = option { // this is used to check intent inside `option`.
        res += x;
    }
    println!("{}", res);
}
