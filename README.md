# Multiple Mutable Borrows in Rust

This example demonstrates a common error in Rust: attempting to create multiple mutable borrows of the same variable.  Rust's borrow checker prevents this to ensure data safety and avoid race conditions.

The `bug.rs` file contains code that will result in a compilation error because it violates Rust's borrowing rules. The `bugSolution.rs` file provides a corrected version.