# Lifo

Simple last-in first-out api wrapper for std `VecDeque<T>`.

## Code

```rust
use std::collections::VecDeque;

pub type Deque<T> = VecDeque<T>;

/// ### Defaults the behavior of `push` and `pop`.
pub trait Lifo<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
}

impl<T> Lifo<T> for Deque<T> {
    /// ### Push back
    #[inline(always)]
    fn push(&mut self, value: T) {
        self.push_back(value);
    }

    /// ### Pop front
    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }
}
```