pub trait VecPopTwo<T> {
    fn pop_two(&mut self) -> Option<(T, T)>;
}

impl<T> VecPopTwo<T> for Vec<T> {
    fn pop_two(&mut self) -> Option<(T, T)> {
        let a = self.pop()?;
        let b = self.pop()?;
        Some((a, b))
    }
}