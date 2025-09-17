pub trait Annotate<A>: Sized {
    type Annotation;
    fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, A>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Annotated<T, A> {
    value: T,
    annotation: A,
}

impl<T, A> Annotated<T, A> {
    pub fn new(value: T, annotation: A) -> Self {
        Self { value, annotation }
    }
    pub fn into_parts(self) -> (T, A) {
        (self.value, self.annotation)
    }
    pub fn parts(&self) -> (&T, &A) {
        (&self.value, &self.annotation)
    }
    pub fn parts_mut(&mut self) -> (&mut T, &mut A) {
        (&mut self.value, &mut self.annotation)
    }
    pub fn into_value(self) -> T {
        self.value
    }
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
    pub fn into_annotation(self) -> A {
        self.annotation
    }
    pub fn annotation(&self) -> &A {
        &self.annotation
    }
    pub fn annotation_mut(&mut self) -> &mut A {
        &mut self.annotation
    }
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Annotated<U, A> {
        Annotated {
            value: f(self.value),
            annotation: self.annotation,
        }
    }
}
