use coalesced::{Coalesce, Extension};

fn main() {
    let a = Some(1).with_extension("a");
    let b = Some(2).with_extension("b".to_string());

    let c = a.coalesce(b);
    assert_eq!(c.value, Some(2));
    assert_eq!(c.extension, "b".to_string());
}
