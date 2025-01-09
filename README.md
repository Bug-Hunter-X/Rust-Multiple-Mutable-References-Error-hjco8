# Rust Multiple Mutable References

This repository demonstrates a common error in Rust related to mutable references.  The `bug.rs` file contains code that may lead to a compiler error due to potential data races.  The `bugSolution.rs` file offers a corrected version.  This example helps illustrate how Rust's borrow checker prevents data races and ensures memory safety.