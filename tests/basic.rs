use coalesced::Coalesced;

pub struct Config<'a> {
    pub name: &'a str,
}

pub fn main() {
    let from_file = Coalesced::new(Some(Config { name: "file" }));
    let from_env = Coalesced::new(Some(Config { name: "env" }));
    let from_cli = Coalesced::new(Some(Config { name: "cli" }));

    let config = from_file.prior(from_env).prior(from_cli);
    assert_eq!(config.as_ref().unwrap().name, "cli");
}
