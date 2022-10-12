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

pub fn load_pollapo_yml(pollapo_yml_path: &str) -> PollapoYml {
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
        let pollapo_yml = load_pollapo_yml("pollapo.test.yml");
        assert_eq!(
            pollapo_yml.root.lock["pbkit/interface-pingpong-server@main"],
            "58425678c6284305dd09130075cecb54a3a3d32c"
        );
    }

    #[test]
    fn load_pollapo_yml_should_load_deps() {
        let pollapo_yml = load_pollapo_yml("pollapo.test.yml");
        assert_eq!(pollapo_yml.deps[0], "pbkit/interface-pingpong-server@main");
    }
}

pub fn parse_dep(dep: &str) -> (&str, &str, &str) {
    let userrepo_ref = dep.split("@").collect::<Vec<&str>>();
    let userrepo = userrepo_ref[0];
    let dep_ref = userrepo_ref[1];
    let user_repo = userrepo.split("/").collect::<Vec<&str>>();
    let user = user_repo[0];
    let repo = user_repo[1];

    (user, repo, dep_ref)
}
