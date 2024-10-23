use std::collections::VecDeque;

/// ### Alias for `VecDeque<T>`.
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

#[cfg(test)]
mod tests {
    use crate::{Deque, Lifo};

    #[test]
    fn test_deque() {
        let mut q = Deque::new();
        for i in 0..100 {
            q.push(i);
        }

        assert_eq!(q.len(), 100);
        assert_eq!(q.front(), Some(&0));

        for i in 0..33 {
            let x = q.pop().expect("Should not be empty!");
            assert_eq!(x, i);
        }

        assert_eq!(q.front(), Some(&33));
    }
}
