#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Annotated<T, P> {
    pub value: T,
    pub annotation: P,
}

impl<T, P> Annotated<T, P> {
    pub fn lift_with(value: T, annotation: P) -> Self {
        Self { value, annotation }
    }
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Annotated<U, P> {
        Annotated {
            value: f(self.value),
            annotation: self.annotation,
        }
    }
}
