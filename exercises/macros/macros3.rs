// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// explanation
// https://stackoverflow.com/a/31749071/7253829

// must state for internal use
// use on modules + parent modules
// imports all macros into main scope
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    // shadowing allowed
    macro_rules! my_macro {
        () => {
            println!("Check out my macro..again!");
        };
    }
}

fn main() {
    my_macro!();
}
