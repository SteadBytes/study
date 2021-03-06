//! Demonstration of various smart pointers through different implementations
//! of a lisp-esque cons list.
mod box_list;
mod rc_list;
mod rc_ref_cell_list;
mod ref_cycle;

fn main() {
    let demo_fns = [
        box_list::demo,
        rc_list::demo,
        rc_ref_cell_list::demo,
        ref_cycle::demo,
    ];
    for demo_fn in demo_fns.iter() {
        demo_fn();
        println!("\n");
    }
}
