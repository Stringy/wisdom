use std::collections::VecDeque;

pub trait VecDequePopTwo<T> {
    fn pop_back_two(&mut self) -> Option<(T, T)>;
}

impl<T> VecDequePopTwo<T> for VecDeque<T> {
    fn pop_back_two(&mut self) -> Option<(T, T)> {
        let a = self.pop_back()?;
        let b = self.pop_back()?;
        Some((a, b))
    }
}