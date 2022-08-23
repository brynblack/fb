use std::ops::Add;

pub struct Fibonacci<T> {
    curr: T,
    next: T,
}

impl<T> Fibonacci<T> {
    pub fn new(curr: T, next: T) -> Self {
        Self { curr, next }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: Add<Output = T> + Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        let next = self.curr + self.next;

        self.curr = self.next;
        self.next = next;

        Some(curr)
    }
}
