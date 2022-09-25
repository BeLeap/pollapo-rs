use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PollapoYml {
    pub deps: Vec<String>,
    pub root: PollapoRoot,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PollapoRoot {
    pub lock: BTreeMap<String, String>,
}

pub fn load_pollapo_yml(pollapo_yml_path: Option<&str>) -> PollapoYml {
    let pollapo_yml_path = match pollapo_yml_path {
        Some(path) => path,
        None => "pollapo.yml",
    };
    let pollapo_yml_content = std::fs::read_to_string(pollapo_yml_path).unwrap_or_else(|err| {
        panic!("Failed to read file {}: {}", pollapo_yml_path, err);
    });
    let pollapo_yml = serde_yaml::from_str(&pollapo_yml_content).unwrap_or_else(|err| {
        panic!("Malfored {}: {}", pollapo_yml_path, err);
    });

    pollapo_yml
}

#[cfg(test)]
mod tests {
    use crate::pollapo_yml::load_pollapo_yml;

    #[test]
    fn load_pollapo_yml_should_load_root_lock() {
        let pollapo_yml = load_pollapo_yml(Some("pollapo.test.yml"));
        assert_eq!(
            pollapo_yml.root.lock["pbkit/interface-pingpong-server@main"],
            "58425678c6284305dd09130075cecb54a3a3d32c"
        );
    }

    #[test]
    fn load_pollapo_yml_should_load_deps() {
        let pollapo_yml = load_pollapo_yml(Some("pollapo.test.yml"));
        assert_eq!(pollapo_yml.deps[0], "pbkit/interface-pingpong-server@main");
    }
}
