use coalesced::{Coalesce, Coalesced};

pub struct Config<'a> {
    pub name: &'a str,
}

pub fn main() {
    let from_file = Coalesced::new(Config { name: "file" });
    let from_env = Coalesced::new(Config { name: "env" });
    let from_cli = Coalesced::new(Config { name: "cli" });

    let config = from_file.coalesce(from_env).coalesce(from_cli);
    assert_eq!(config.name, "cli");
}
