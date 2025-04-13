use crate::extension::{Extension, WithExt};
#[doc = "Generated implementation"]
impl<X> Extension<X> for () {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &() {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut () {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for bool {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &bool {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut bool {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for char {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &char {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut char {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for String {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &String {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut String {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for usize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &usize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut usize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for u8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &u8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut u8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for u16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &u16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut u16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for u32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &u32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut u32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for u64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &u64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut u64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for u128 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &u128 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut u128 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for isize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &isize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut isize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for i8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &i8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut i8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for i16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &i16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut i16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for i32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &i32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut i32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for i64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &i64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut i64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for i128 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &i128 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut i128 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for f32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &f32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut f32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for f64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &f64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut f64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &str {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::path::Path {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::path::PathBuf {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::path::PathBuf {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::path::PathBuf {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::time::Duration {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::time::Duration {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::time::Duration {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::time::Instant {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::time::Instant {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::time::Instant {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::time::SystemTime {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::time::SystemTime {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::time::SystemTime {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::net::IpAddr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::net::IpAddr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::net::IpAddr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::net::Ipv4Addr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::net::Ipv4Addr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::net::Ipv4Addr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::net::Ipv6Addr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::net::Ipv6Addr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::net::Ipv6Addr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::net::SocketAddr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::net::SocketAddr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::net::SocketAddr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::net::SocketAddrV4 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::net::SocketAddrV4 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::net::SocketAddrV4 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::net::SocketAddrV6 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::net::SocketAddrV6 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::net::SocketAddrV6 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicBool {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicBool {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicBool {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicIsize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicIsize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicIsize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicI8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicI8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicI8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicI16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicI16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicI16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicI32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicI32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicI32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicI64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicI64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicI64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicUsize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicUsize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicUsize {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicU8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicU8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicU8 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicU16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicU16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicU16 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicU32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicU32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicU32 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::sync::atomic::AtomicU64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::sync::atomic::AtomicU64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::sync::atomic::AtomicU64 {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::ffi::OsStr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::ffi::OsString {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::ffi::OsString {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::ffi::OsString {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::ffi::CStr {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for std::ffi::CString {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &std::ffi::CString {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X> Extension<X> for &mut std::ffi::CString {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::marker::PhantomData<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::marker::PhantomData<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::marker::PhantomData<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for Box<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &Box<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut Box<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::rc::Rc<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::rc::Rc<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::rc::Rc<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::rc::Weak<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::rc::Weak<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::rc::Weak<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::sync::Arc<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::sync::Arc<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::sync::Arc<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::sync::Weak<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::sync::Weak<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::sync::Weak<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::sync::Mutex<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::sync::Mutex<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::sync::Mutex<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::sync::RwLock<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::sync::RwLock<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::sync::RwLock<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::cell::Cell<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::cell::Cell<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::cell::Cell<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::cell::RefCell<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::cell::RefCell<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::cell::RefCell<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::num::Saturating<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::num::Saturating<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::num::Saturating<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::num::Wrapping<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::num::Wrapping<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::num::Wrapping<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for std::cmp::Reverse<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &std::cmp::Reverse<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
#[doc = "Generated implementation"]
impl<X, T> Extension<X> for &mut std::cmp::Reverse<T> {
    type WithExt = WithExt<Self, X>;
    fn with_extension(self, extension: X) -> Self::WithExt {
        WithExt {
            value: self,
            extension,
        }
    }
    fn unwrap_extension(with_ext: Self::WithExt) -> Self {
        with_ext.value
    }
    fn ex_prior(_base: Self::WithExt, other: Self::WithExt) -> Self::WithExt {
        other
    }
    fn ex_posterior(base: Self::WithExt, _other: Self::WithExt) -> Self::WithExt {
        base
    }
}
