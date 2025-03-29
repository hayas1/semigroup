use crate::extension::{Extension, WithExt};
#[doc = "Generated implementation"]
impl Extension for () {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &() {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut () {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for bool {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &bool {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut bool {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for char {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &char {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut char {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for String {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &String {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut String {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for u8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &u8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut u8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for u16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &u16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut u16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for u32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &u32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut u32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for u64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &u64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut u64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for u128 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &u128 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut u128 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for i8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &i8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut i8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for i16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &i16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut i16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for i32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &i32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut i32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for i64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &i64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut i64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for i128 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &i128 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut i128 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for f32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &f32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut f32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for f64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &f64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut f64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &str {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::path::Path {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::path::PathBuf {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::path::PathBuf {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::path::PathBuf {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::time::Duration {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::time::Duration {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::time::Duration {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::time::Instant {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::time::Instant {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::time::Instant {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::time::SystemTime {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::time::SystemTime {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::time::SystemTime {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::net::IpAddr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::net::IpAddr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::net::IpAddr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::net::Ipv4Addr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::net::Ipv4Addr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::net::Ipv4Addr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::net::Ipv6Addr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::net::Ipv6Addr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::net::Ipv6Addr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::net::SocketAddr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::net::SocketAddr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::net::SocketAddr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::net::SocketAddrV4 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::net::SocketAddrV4 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::net::SocketAddrV4 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::net::SocketAddrV6 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::net::SocketAddrV6 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::net::SocketAddrV6 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicBool {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicBool {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicBool {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicIsize {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicIsize {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicIsize {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicI8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicI8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicI8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicI16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicI16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicI16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicI32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicI32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicI32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicI64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicI64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicI64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicUsize {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicUsize {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicUsize {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicU8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicU8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicU8 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicU16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicU16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicU16 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicU32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicU32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicU32 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::sync::atomic::AtomicU64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::sync::atomic::AtomicU64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::sync::atomic::AtomicU64 {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::ffi::OsStr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::ffi::OsString {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::ffi::OsString {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::ffi::OsString {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::ffi::CStr {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for std::ffi::CString {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &std::ffi::CString {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl Extension for &mut std::ffi::CString {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::marker::PhantomData<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::marker::PhantomData<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::marker::PhantomData<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for Box<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &Box<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut Box<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::rc::Rc<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::rc::Rc<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::rc::Rc<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::rc::Weak<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::rc::Weak<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::rc::Weak<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::sync::Arc<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::sync::Arc<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::sync::Arc<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::sync::Weak<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::sync::Weak<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::sync::Weak<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::sync::Mutex<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::sync::Mutex<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::sync::Mutex<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::sync::RwLock<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::sync::RwLock<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::sync::RwLock<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::cell::Cell<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::cell::Cell<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::cell::Cell<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::cell::RefCell<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::cell::RefCell<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::cell::RefCell<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::num::Saturating<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::num::Saturating<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::num::Saturating<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::num::Wrapping<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::num::Wrapping<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::num::Wrapping<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for std::cmp::Reverse<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &std::cmp::Reverse<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
#[doc = "Generated implementation"]
impl<T> Extension for &mut std::cmp::Reverse<T> {
    type WithExt<X> = WithExt<Self, X>;
    fn with_extension<X>(self, extension: X) -> Self::WithExt<X> {
        WithExt {
            value: self,
            extension,
        }
    }
    fn ex_prior<X>(_base: WithExt<Self, X>, other: WithExt<Self, X>) -> WithExt<Self, X> {
        other
    }
    fn ex_posterior<X>(base: WithExt<Self, X>, _other: WithExt<Self, X>) -> WithExt<Self, X> {
        base
    }
}
