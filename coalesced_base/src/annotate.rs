pub trait Annotate<A>: Sized {
    type Annotation;
    fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, A>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Annotated<T, A> {
    pub value: T,
    pub annotation: A,
}

impl<T, A> Annotated<T, A> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Annotated<U, A> {
        Annotated {
            value: f(self.value),
            annotation: self.annotation,
        }
    }
}
