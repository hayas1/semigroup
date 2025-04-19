use coalesced::Coalesce;

#[derive(Coalesce)]
union UnionType {
    b: u32,
    f: f32,
}

fn main() {}
