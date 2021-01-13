// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

fn main() {
    // explicit
    let mut shopping_list: Vec<&str> = Vec::new();

    // inferred
    // let mut shopping_list = vec![];

    shopping_list.push("milk");
}
