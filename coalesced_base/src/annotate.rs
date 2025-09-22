use std::ops::{Deref, DerefMut};

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
    pub fn as_ref(&self) -> Annotated<&T, &A> {
        Annotated {
            value: &self.value,
            annotation: &self.annotation,
        }
    }
    pub fn as_ref_mut(&mut self) -> Annotated<&mut T, &mut A> {
        Annotated {
            value: &mut self.value,
            annotation: &mut self.annotation,
        }
    }
    pub fn as_deref(&self) -> Annotated<&T::Target, &A>
    where
        T: Deref,
    {
        self.as_ref().map(|v| v.deref())
    }
    pub fn as_deref_mut(&mut self) -> Annotated<&mut T::Target, &mut A>
    where
        T: DerefMut,
    {
        self.as_ref_mut().map(|v| v.deref_mut())
    }
}
impl<T, A> Annotated<&T, &A> {
    pub fn cloned(self) -> Annotated<T, A>
    where
        T: Clone,
        A: Clone,
    {
        Annotated {
            value: self.value.clone(),
            annotation: self.annotation.clone(),
        }
    }
    pub fn copied(self) -> Annotated<T, A>
    where
        T: Copy,
        A: Copy,
    {
        Annotated {
            value: *self.value,
            annotation: *self.annotation,
        }
    }
}
impl<T, A> Annotated<&T, A> {
    pub fn value_cloned(self) -> Annotated<T, A>
    where
        T: Clone,
    {
        Annotated {
            value: self.value.clone(),
            annotation: self.annotation,
        }
    }
    pub fn value_copied(self) -> Annotated<T, A>
    where
        T: Copy,
    {
        Annotated {
            value: *self.value,
            annotation: self.annotation,
        }
    }
}
impl<T, A> Annotated<T, &A> {
    pub fn annotation_cloned(self) -> Annotated<T, A>
    where
        A: Clone,
    {
        Annotated {
            value: self.value,
            annotation: self.annotation.clone(),
        }
    }
    pub fn annotation_copied(self) -> Annotated<T, A>
    where
        A: Copy,
    {
        Annotated {
            value: self.value,
            annotation: *self.annotation,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let one = Annotated::new(1, "first");
        let sone = one.map(|i| i.to_string());
        assert_eq!(
            sone.as_deref().annotation_cloned(),
            Annotated::new("1", "first")
        );
    }
}
