//! `#[properties]` takes no arguments: it reads the `#[semigroup_op(..)]` or `#[semigroup(..)]`
//! of the same item, see [`crate::attr`].
use syn::{DeriveInput, ItemStruct};

use crate::{attr::CommonAttr, op, semigroup};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ContainerAttr {
    common: CommonAttr,
}
impl ContainerAttr {
    /// An item without either attribute has no properties beyond [`Semigroup`](crate).
    /// An attribute the derive rejects is reported by the derive, so it reads as no properties here.
    pub fn from_item(item: &ItemStruct) -> Self {
        let derive = DeriveInput::from(item.clone());
        let has = |name| item.attrs.iter().any(|a| a.path().is_ident(name));
        let common = if has("semigroup_op") {
            op::attr::ContainerAttr::new(&derive)
                .ok()
                .map(|a| a.common().clone())
        } else if has("semigroup") {
            semigroup::attr::ContainerAttr::new(&derive)
                .ok()
                .map(|a| a.common().clone())
        } else {
            None
        };
        Self {
            common: common.unwrap_or_default(),
        }
    }
    pub fn common(&self) -> &CommonAttr {
        &self.common
    }
}
