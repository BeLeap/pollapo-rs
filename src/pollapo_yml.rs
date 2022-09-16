use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct PollapoYml {
    root: PollapoRoot
}

#[derive(Debug, PartialEq)]
pub struct PollapoRoot {
    lock: BTreeMap<String, String>
}

pub fn load_pollapo_yml() -> PollapoYml {
    PollapoYml { root: PollapoRoot { lock: BTreeMap::new() } }
}

#[cfg(test)]
mod tests {
    use crate::pollapo_yml::load_pollapo_yml;

    #[test]
    fn load_pollapo_yml_should_load_root_lock() {
        let pollapo_yml = load_pollapo_yml();
        assert!(pollapo_yml.root.lock["pbkit/interface-pingpong-server@main"] == "58425678c6284305dd09130075cecb54a3a3d32c");
    }
}
