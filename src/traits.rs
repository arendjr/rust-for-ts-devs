use crate::copy_trait::MyCopyableStruct;

// Traits are defined with the `trait` keyword and are similar
// to TypeScript interfaces with method signatures.

/// Trimmed version of `std::ops::Add`.
pub trait Add<Rhs = Self> {
    /// The resulting type after applying the `+` operator.
    type Output; // <-- This is an *associated type*.

    /// Performs the `+` operation.
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl std::ops::Add for MyCopyableStruct {
    type Output = MyCopyableStruct;

    fn add(self, rhs: Self) -> Self::Output {
        MyCopyableStruct {
            foo: self.foo + rhs.foo,
        }
    }
}
