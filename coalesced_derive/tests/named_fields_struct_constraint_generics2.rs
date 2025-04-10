// TODO required phantom data and conflict type parameter

// use std::fmt::Display;
// use std::marker::PhantomData;

// use coalesced::Coalesce;

// use coalesced::Extension;
// use coalesced_derive::Extension;

// #[derive(Extension)]
// struct Config<S, T: Clone, E>
// where
//     S: Display + Extension<E>,
// {
//     name: S,
//     value: Option<T>,
//     phantom: PhantomData<E>,
// }

// #[test]
// fn test_derive_named_fields_struct_constraint_generics() {
//     let config = Config {
//         name: "c1",
//         value: Some(1),
//         phantom: PhantomData::<()>,
//     };
//     let config2 = Config {
//         name: "c2",
//         value: None,
//         phantom: PhantomData::<()>,
//     };

//     let c = config.prior(config2);
//     assert_eq!(c.name, "c2");
//     assert_eq!(c.value, Some(1));
// }

// #[test]
// fn test_derive_extension_named_fields_struct_constraint_generics() {
//     let config = Config {
//         name: "c1",
//         value: None,
//         phantom: PhantomData::<()>,
//     }
//     .with_extension("first");
//     let config2 = Config {
//         name: "c2",
//         value: Some(2),
//         phantom: PhantomData::<()>,
//     }
//     .with_extension("second");

//     let c = config.posterior(config2);
//     assert_eq!(c.name.extension, "first");
//     assert_eq!(*c.name, "c1");
//     assert_eq!(c.value.extension, "second");
//     assert_eq!(*c.value, Some(2));

//     assert!(matches!(
//         c.into(),
//         Config {
//             name: "c1",
//             value: Some(2),
//             ..
//         }
//     ));
// }
