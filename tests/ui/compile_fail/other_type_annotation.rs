use coalesced::{op::annotation::coalesce::Coalesce, Annotate};

fn main() {
    let a = Some(1).annotated("a");
    let b = Some(2).annotated("b".to_string());

    let c = a.coalesce(b);
    assert_eq!(c.value, Some(2));
    assert_eq!(c.annotation, "b".to_string());
}
