//! Demonstration of callstack operation by printing "progress" (as the depth
//! of recursive function calls) throughout program execution.
//!
//! Note: Primarily intended to aid understanding of the OS `longjmp`
//! facilities demonstrated in `sjlj` program from this chapter.
fn print_depth(depth: usize) {
    println!("{}", "#".repeat(depth));
}

fn dive(depth: usize, max_depth: usize) {
    print_depth(depth);
    if depth >= max_depth {
        return;
    }
    dive(depth + 1, max_depth);
    print_depth(depth);
}

fn main() {
    dive(0, 5);
}
