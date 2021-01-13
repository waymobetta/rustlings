// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13)); 
    print_number(Some(99));
    
    // leverage default crate to init a fixed-length array of u16 up to 32 items
    // let mut numbers: [Option<u16>; 5] = Default::default();
    
    // or use None
    let mut numbers: [Option<u16>; 5] = [None; 5];

    for iter in 0..5 {
        let number_to_add: Option<u16> = Some(((iter * 1235) + 2) / (4 * 16));

        numbers[iter as usize] = number_to_add;
    }
}
