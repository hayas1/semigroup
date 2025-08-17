pub trait Annotate: Sized {
    fn annotated<P>(self, annotation: P) -> Annotated<Self, P>;
}
impl<T> Annotate for T {
    fn annotated<P>(self, annotation: P) -> Annotated<Self, P> {
        Annotated {
            value: self,
            annotation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Annotated<T, P> {
    pub value: T,
    pub annotation: P,
}

impl<T, P> Annotated<T, P> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Annotated<U, P> {
        Annotated {
            value: f(self.value),
            annotation: self.annotation,
        }
    }
}
