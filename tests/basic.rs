use coalesced::Coalesced;

pub struct Config<'a> {
    pub name: &'a str,
}

pub fn main() {
    let from_file = Coalesced::new_prior(Some(Config { name: "file" }));
    let from_env = Coalesced::new_prior(Some(Config { name: "env" }));
    let from_cli = Coalesced::new_prior(Some(Config { name: "cli" }));

    let config = from_file.extend_prior(from_env).extend_prior(from_cli);
    assert_eq!(config.as_ref().unwrap().name, "cli");
}
