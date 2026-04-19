use semigroup::Semigroup;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Semigroup)]
pub struct BasicConfig {
    #[semigroup(with = "semigroup::op::Coalesce")]
    pub first: Option<u32>,
    #[semigroup(with = "semigroup::op::Concat")]
    pub list: Vec<String>,
}

#[test]
fn test_basic_serde() {
    let conf1 = r#"{"first": null, "list": []}"#;
    let conf2 = r#"{"first": 1, "list": ["a", "b"]}"#;
    let conf3 = r#"{"first": 2, "list": ["c", "d"]}"#;

    let c1: BasicConfig = serde_json::from_str(conf1).unwrap();
    let c2: BasicConfig = serde_json::from_str(conf2).unwrap();
    let c3: BasicConfig = serde_json::from_str(conf3).unwrap();

    let config = c1.semigroup(c2).semigroup(c3);
    assert_eq!(
        config,
        BasicConfig {
            first: Some(1),
            list: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ]
        }
    );
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Semigroup)]
pub struct AdvancedConfig {
    #[semigroup(with = "semigroup::op::Coalesce")]
    pub first: Option<u32>,
    #[semigroup(with = "semigroup::Dual(semigroup::op::Concat(_))")]
    pub list: Vec<String>,
}

#[test]
fn test_serde_dual() {
    let conf1 = r#"{"first": null, "list": []}"#;
    let conf2 = r#"{"first": 1, "list": ["a", "b"]}"#;
    let conf3 = r#"{"first": 2, "list": ["c", "d"]}"#;

    let c1: AdvancedConfig = serde_json::from_str(conf1).unwrap();
    let c2: AdvancedConfig = serde_json::from_str(conf2).unwrap();
    let c3: AdvancedConfig = serde_json::from_str(conf3).unwrap();

    let config = c1.semigroup(c2).semigroup(c3);
    assert_eq!(
        config,
        AdvancedConfig {
            first: Some(1),
            list: vec![
                "c".to_string(),
                "d".to_string(),
                "a".to_string(),
                "b".to_string(),
            ]
        }
    );
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Semigroup)]
pub struct DualDualConfig {
    #[semigroup(with = "semigroup::op::Coalesce")]
    pub first: Option<u32>,
    #[semigroup(with = "semigroup::Dual(semigroup::Dual(semigroup::op::Concat(_)))")]
    pub list: Vec<String>,
}

#[test]
fn test_serde_test_config() {
    let conf1 = r#"{"first": null, "list": []}"#;
    let conf2 = r#"{"first": 1, "list": ["a", "b"]}"#;
    let conf3 = r#"{"first": 2, "list": ["c", "d"]}"#;

    let bc1: BasicConfig = serde_json::from_str(conf1).unwrap();
    let bc2: BasicConfig = serde_json::from_str(conf2).unwrap();
    let bc3: BasicConfig = serde_json::from_str(conf3).unwrap();
    let basic_config = bc1.semigroup(bc2).semigroup(bc3);

    let ddc1: DualDualConfig = serde_json::from_str(conf1).unwrap();
    let ddc2: DualDualConfig = serde_json::from_str(conf2).unwrap();
    let ddc3: DualDualConfig = serde_json::from_str(conf3).unwrap();
    let dual_dual_config = ddc1.semigroup(ddc2).semigroup(ddc3);

    assert_eq!(basic_config.first, dual_dual_config.first);
    assert_eq!(basic_config.list, dual_dual_config.list);
}
