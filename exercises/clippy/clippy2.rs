// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // improve readability
    if let Some(x) = option {
        res += x;
    }

    println!("{}", res);
}
