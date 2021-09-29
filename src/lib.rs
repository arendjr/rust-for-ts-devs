#![allow(clippy::blacklisted_name)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::new_without_default)]
#![allow(dead_code)]

mod enums;
mod functions_and_tuples;
mod if_else;
mod r#match;
mod numbers;
mod option_and_result;
mod option_and_result_conversions;
mod strings;
mod struct_impl;
mod structs;

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
