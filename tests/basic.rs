// use coalesced::{Coalesce, CoalesceExt};

// pub struct Config<'a> {
//     pub name: &'a str,
// }

// #[test]
// fn test_coalesced_basic_config() {
//     let from_file = Some(Config { name: "file" });
//     let from_env = Some(Config { name: "env" });
//     let from_cli = Some(Config { name: "cli" });

//     let config = from_file.coalesce(from_env).coalesce(from_cli).prior();
//     assert_eq!(config.as_ref().unwrap().name, "cli");
// }

// struct GlobalConfig<'a> {
//     _name: &'a str,
//     number: Option<i64>,
//     locals: Vec<LocalConfig<'a>>,
// }
// struct LocalConfig<'a> {
//     _name: &'a str,
//     number: Option<i64>,
// }
// #[test]
// fn test_coalesced_complex_config() {
//     let config = GlobalConfig {
//         _name: "global",
//         number: Some(1),
//         locals: vec![
//             LocalConfig {
//                 _name: "local1",
//                 number: Some(10),
//             },
//             LocalConfig {
//                 _name: "local2",
//                 number: Some(100),
//             },
//             LocalConfig {
//                 _name: "local3",
//                 number: None,
//             },
//         ],
//     };
//     let base = config.number.prior();
//     let expected = [Some(10), Some(100), Some(1)];
//     for (cfg, exp) in config.locals.into_iter().zip(expected) {
//         let number = base.clone().coalesce(cfg.number);
//         assert_eq!(*number, exp);
//     }
// }
