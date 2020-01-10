// Each file in tests directory is a separate crate -> need to import modules
// as of using 'publicly'
use adder;

mod common; // Import common test helpers lib

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
