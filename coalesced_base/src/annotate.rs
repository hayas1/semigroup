pub trait Annotate: Sized {
    fn annotated<A>(self, annotation: A) -> Annotated<Self, A>;
}
impl<T> Annotate for T {
    fn annotated<A>(self, annotation: A) -> Annotated<Self, A> {
        Annotated {
            value: self,
            annotation,
        }
    }
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
    pub fn extract<U, B>(
        self,
        fv: impl FnOnce(T) -> U,
        fa: impl FnOnce(A) -> B,
    ) -> Annotated<U, B> {
        Annotated {
            value: fv(self.value),
            annotation: fa(self.annotation),
        }
    }
}
