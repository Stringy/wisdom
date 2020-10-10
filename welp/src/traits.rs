pub trait Executable<R> {
    type Err;
    fn execute(&self) -> Result<R, Self::Err>;
}

pub struct WisdomObject<T> {
    kind: ObjectKind,
    value: T,
}

enum ObjectKind {}