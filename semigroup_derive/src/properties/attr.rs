use darling::FromMeta;

#[derive(Debug, Clone, PartialEq, FromMeta)]
#[darling(derive_syn_parse, and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    selective: bool,
    selective_where: Option<String>,

    #[darling(default)]
    monoid: bool,
    monoid_where: Option<String>,

    #[darling(default)]
    commutative: bool,
    commutative_where: Option<String>,
}
impl ContainerAttr {
    pub fn validate(self) -> darling::Result<Self> {
        Ok(self)
    }
    pub fn is_selective(&self) -> bool {
        self.selective
    }
    pub fn is_monoid(&self) -> bool {
        self.monoid
    }
    pub fn is_commutative(&self) -> bool {
        self.commutative
    }

    pub fn selective_where(&self) -> Option<&str> {
        self.selective_where.as_deref()
    }
    pub fn monoid_where(&self) -> Option<&str> {
        self.monoid_where.as_deref()
    }
    pub fn commutative_where(&self) -> Option<&str> {
        self.commutative_where.as_deref()
    }
}
