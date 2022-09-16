use clap::Parser;

use crate::pollapo_yml::PollapoYml;

#[derive(Parser, Debug)]
#[clap(about = "Install dependencies")]
pub struct InstallArgs {}

pub fn action() {
    todo!()
}

async fn install_dep_to_cache(
    pollapo_yml: &PollapoYml,
    dep: &str,
    cache_dir: Option<&str>,
) -> std::path::PathBuf {
    let target_hash = &pollapo_yml.root.lock[dep];
    let repo_name = dep.split("@").collect::<Vec<&str>>()[0];
    let zipball_url = format!("https://github.com/{}/zipball/{}", repo_name, target_hash);
    let response = reqwest::get(&zipball_url).await
        .unwrap_or_else(|err| {
            panic!("Failed to fetch {}: {}", &zipball_url, err);
        });

    let mut content = std::io::Cursor::new(response.bytes().await.unwrap_or_else(|err| {
        panic!("Failed to convert {} into bytes: {}", &zipball_url, err);
    }));

    let cache_dir = match cache_dir {
        Some(dir) => dir,
        None => "~/.config/pollapo/cache"
    };
    let file_name = format!("{}/{}.{}", cache_dir, dep, "zip");
    let file_path = std::path::Path::new(&file_name);
    let file_parent = file_path.parent().unwrap();
    std::fs::create_dir_all(&file_parent)
        .unwrap_or_else(|err| {
            panic!("Failed to create parent {} of {}: {}", &file_parent.to_string_lossy(), &file_name, err);
        });
    let mut target_file = std::fs::File::create(&file_name)
        .unwrap_or_else(|err| {
            panic!("Failed to create {}: {}", &file_name, err);
        });

    std::io::copy(&mut content, &mut target_file)
        .unwrap_or_else(|err| {
            panic!("Failed to copy {} to {}: {}", zipball_url, file_name, err);
        });

    file_path.to_path_buf()
}

#[cfg(test)]
mod tests {
    use crate::pollapo_yml::load_pollapo_yml;
    use super::install_dep_to_cache;

    #[test]
    fn install_dep_to_cache_should_store_dep_zip_to_cache() {
        let pollapo_yml = load_pollapo_yml(Some("pollapo.test.yml"));
        let path = tokio_test::block_on(
            install_dep_to_cache(
                &pollapo_yml,
                "pbkit/interface-pingpong-server@main", 
                Some("cache_test"),
            )
        );

        let expected_path = std::path::PathBuf::from("cache_test/pbkit/interface-pingpong-server@main.zip");
        assert_eq!(path, expected_path);
        assert!(
            std::fs::read_dir("./cache_test/pbkit").unwrap()
                .map(|entry| entry.unwrap().path())
                .fold(false, |acc, entry| acc || entry.to_string_lossy().contains("pbkit/interface-pingpong-server@main"))
        );

        std::fs::remove_file("./cache_test/pbkit/interface-pingpong-server@main.zip").unwrap();
    }
}
