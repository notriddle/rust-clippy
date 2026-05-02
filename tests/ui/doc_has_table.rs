#![warn(clippy::doc_has_table)]

/// | Not | A | Table
pub fn a() {}

/// | Is | Table |
/// |----|-------|
/// | in | here  |
//~^^^ doc_has_table
pub fn b() {}

fn main() {
    // test code goes here
}
