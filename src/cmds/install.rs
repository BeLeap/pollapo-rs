use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about = "Install dependencies")]
pub struct InstallArgs {}

pub fn action() {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn install_dep_should_store_dep_zip_to_target() {
        install_dep("pbkit/interface-pingpong-server@main", Some("cache_test"));
        let temp = std::fs::read_dir("cache_test").unwrap()
            .map(|entry| entry.unwrap().path()).collect::<Vec<PathBuf>>()
            .contains(&std::path::PathBuf::from("./cache_test/pbkit/interface-pingpong-server@main.zip"));
    }
}
