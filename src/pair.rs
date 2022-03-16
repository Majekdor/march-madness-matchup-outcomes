/// # Pair Struct
///
/// Stores two related values of the same type. This struct is needed due to the
/// implementation's `get()` function intended to be used in a loop.
///
/// If the `get()` function is not needed just use a standard tuple.
pub struct Pair<T> {
    first: T,
    second: T,
}

impl <T> Pair<T> {
    /// Create a new pair from two values.
    pub fn from(first: T, second: T) -> Pair<T> {
        Pair {
            first,
            second,
        }
    }

    /// Get the first value of the pair.
    pub fn first(&self) -> &T {
        &self.first
    }

    /// Get the second value of the pair.
    pub fn second(&self) -> &T {
        &self.second
    }

    /// Get one of the elements of the pair. The index must be 1 or 2.
    ///
    /// This function is useful in loops like this:
    /// ```rust
    /// let pair = Pair::from(10, 20);
    /// for i 1..3 {
    ///     // Returns 10 then 20
    ///     pair.get(i);
    /// }
    /// ```
    pub fn get(&self, index: i32) -> &T {
        match index {
            1 => self.first(),
            2 => self.second(),
            _ => panic!("Invalid index for pair: {}", index),
        }
    }
}