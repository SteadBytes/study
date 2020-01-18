// Demonstration of procedural macros via a custom `derive` macro
// See `projects/hello_macro` for the macro definition
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
