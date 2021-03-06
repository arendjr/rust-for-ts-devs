#![allow(clippy::blacklisted_name)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::new_without_default)]
#![allow(dead_code)]

mod borrowing;
mod borrowing_rules;
mod boxes;
mod callbacks;
mod cells;
mod clone_trait;
mod copy_trait;
mod enums;
mod function_traits;
mod functions_and_tuples;
mod generic_functions;
mod generic_structs;
mod if_else;
mod impl_and_dyn;
mod let_bindings;
mod lifetime_elision;
mod lifetimes;
mod maps;
mod r#match;
mod move_semantics;
mod mutability;
mod mutable_methods;
mod numbers;
mod option_and_result;
mod option_and_result_conversions;
mod reference_counting;
mod references;
mod sets;
mod split_examples;
mod strings;
mod strings_revisited;
mod struct_impl;
mod structs;
mod traits;
mod vectors;
mod where_clauses;

// First course: Basic data types and control flow
mod course_1 {
    // 1: Numbers
    pub use super::numbers::*;
    // 2: Strings
    pub use super::strings::*;
    // 3: Structs
    pub use super::structs::*;
    // 4: Functions and tuples
    pub use super::functions_and_tuples::*;
    // 5: Struct impls
    pub use super::struct_impl::*;
    // 6: Enums
    pub use super::enums::*;
    // 7: If-else
    pub use super::if_else::*;
    // 8: Match
    pub use super::r#match::*;
    // 9: Option and Result
    pub use super::option_and_result::*;
    // 10: Conversions between Option and Result
    pub use super::option_and_result_conversions::*;
}

// Second course: Mutability and references
mod course_2 {
    // 1: Let bindings
    pub use super::let_bindings::*;
    // 2: Move semantics
    pub use super::move_semantics::*;
    // 3: Clone
    pub use super::clone_trait::*;
    // 4: Copy
    pub use super::copy_trait::*;
    // 5: Mutability
    pub use super::mutability::*;
    // 6: Mutable methods
    pub use super::mutable_methods::*;
    // 7: References
    pub use super::references::*;
    // 8: Borrowing
    pub use super::borrowing::*;
    // 9: Borrowing rules
    pub use super::borrowing_rules::*;
    // 10: Strings revisited
    pub use super::strings_revisited::*;
}

// Third course: Generics and lifetimes
mod course_3 {
    // 1: Generic structs
    pub use super::generic_structs::*;
    // 2: Generic functions
    pub use super::generic_functions::*;
    // 3: Where clauses
    pub use super::where_clauses::*;
    // 4: Traits
    pub use super::traits::*;
    // 5: Impl and dyn
    pub use super::impl_and_dyn::*;
    // 6: Lifetimes
    pub use super::lifetimes::*;
    // 7: Lifetime elision
    pub use super::lifetime_elision::*;
    // 8: Split examples
    pub use super::split_examples::*;
}

// Fourth course: Containers and callbacks
mod course_4 {
    // 1: Vectors
    pub use super::vectors::*;
    // 2: Sets
    pub use super::sets::*;
    // 3: Maps
    pub use super::maps::*;
    // 4: Box
    pub use super::boxes::*;
    // 5: Reference counting
    pub use super::reference_counting::*;
    // 6: Cells
    pub use super::cells::*;
    // 7: Callbacks
    pub use super::callbacks::*;
    // 8: Function traits
    pub use super::function_traits::*;
}

mod looking_ahead;

fn main() {
    println!("# Course 2: Quiz answers");
    mutability::quiz();
    references::second_quiz();
}
