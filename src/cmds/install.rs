use clap::Parser;
use piz::read::*;

use crate::{pollapo_yml::PollapoYml, utils::archive::strip};

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
    let response = reqwest::get(&zipball_url).await.unwrap_or_else(|err| {
        panic!("Failed to fetch {}: {}", &zipball_url, err);
    });

    let mut content = std::io::Cursor::new(response.bytes().await.unwrap_or_else(|err| {
        panic!("Failed to convert {} into bytes: {}", &zipball_url, err);
    }));

    let cache_dir_raw = match cache_dir {
        Some(dir) => dir,
        None => "~/.config/pollapo/cache",
    };
    let cache_dir = shellexpand::full(cache_dir_raw).unwrap_or_else(|err| {
        panic!("Failed to resolve {}: {}", cache_dir_raw, err);
    });
    let file_name = format!("{}/{}@{}.{}", cache_dir, repo_name, target_hash, "zip");
    let file_path = std::path::Path::new(&file_name);
    let file_parent = file_path.parent().unwrap();
    std::fs::create_dir_all(&file_parent).unwrap_or_else(|err| {
        panic!(
            "Failed to create parent {} of {}: {}",
            &file_parent.to_string_lossy(),
            &file_name,
            err
        );
    });
    let mut target_file = std::fs::File::create(&file_name).unwrap_or_else(|err| {
        panic!("Failed to create {}: {}", &file_name, err);
    });

    std::io::copy(&mut content, &mut target_file).unwrap_or_else(|err| {
        panic!("Failed to copy {} to {}: {}", zipball_url, file_name, err);
    });

    file_path.to_path_buf()
}

fn extract_cache(path: &std::path::PathBuf, target_dir: Option<&str>) {
    let target_dir_raw = match target_dir {
        Some(dir) => dir,
        None => "~/.config/pollapo/cache",
    };
    let target_dir_str = shellexpand::full(target_dir_raw).unwrap_or_else(|err| {
        panic!("Failed to resolve {}: {}", target_dir_raw, err);
    });
    let target_dir = std::path::Path::new(&*target_dir_str);

    let zipball_file = std::fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to open {}: {}", path.to_string_lossy(), err);
    });
    let zipball = piz::ZipArchive::new(&zipball_file).unwrap_or_else(|err| {
        panic!("Malformed zipball {}: {}", path.to_string_lossy(), err);
    });

    let zip_tree = as_tree(zipball.entries()).unwrap_or_else(|err| {
        panic!(
            "Malfored zipball converting {}: {}",
            path.to_string_lossy(),
            err
        );
    });
    zip_tree.files().for_each(|entry| {
        let full_path = target_dir.join(strip(&*entry.path, 1));

        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|err| {
                panic!(
                    "Failed to create parent {}: {}",
                    parent.to_string_lossy(),
                    err
                );
            });
        }
        let mut reader = zipball.read(entry).unwrap_or_else(|err| {
            panic!(
                "Failed to read entry {}: {}",
                full_path.to_string_lossy(),
                err
            );
        });
        let mut out = std::fs::File::create(&full_path).unwrap_or_else(|err| {
            panic!(
                "Failed to create file {}: {}",
                full_path.to_string_lossy(),
                err
            );
        });
        std::io::copy(&mut reader, &mut out).unwrap_or_else(|err| {
            panic!("Failed to create {}: {}", full_path.to_string_lossy(), err);
        });
    });
}

#[cfg(test)]
mod tests {
    use serial_test::file_serial;

    use super::install_dep_to_cache;
    use crate::{cmds::install::extract_cache, pollapo_yml::load_pollapo_yml};

    #[test]
    #[file_serial(key, "cache_test/pbkit/interface-pingpong-server@main.zip")]
    fn extract_cache_should_extract_from_cache_to_target() {
        // given
        let pollapo_yml = load_pollapo_yml(Some("pollapo.test.yml"));
        let path = tokio_test::block_on(install_dep_to_cache(
            &pollapo_yml,
            "pbkit/interface-pingpong-server@main",
            Some("cache_test"),
        ));

        // when
        extract_cache(&path, Some(".pollapo"));

        // then
        assert!(
            std::fs::read_dir("./.pollapo/pbkit/interface-pingpong-server")
                .unwrap()
                .map(|e| e.unwrap().path())
                .fold(false, |acc, entry| acc
                    || entry.to_string_lossy().contains("pingpong.proto"))
        );

        // clean
        // std::fs::remove_dir_all("./cache_test/pbkit/interface-pingpong-server@main.zip").unwrap();
        // std::fs::remove_dir_all("./.pollapo/pbkit/interface-pingpong-server").unwrap();
    }

    #[test]
    #[file_serial(key, "cache_test/pbkit/interface-pingpong-server@main.zip")]
    fn install_dep_to_cache_should_store_dep_zip_to_cache() {
        // given
        let pollapo_yml = load_pollapo_yml(Some("pollapo.test.yml"));

        // when
        let path = tokio_test::block_on(install_dep_to_cache(
            &pollapo_yml,
            "pbkit/interface-pingpong-server@main",
            Some("cache_test"),
        ));

        // then
        let expected_path =
            std::path::PathBuf::from("cache_test/pbkit/interface-pingpong-server@main.zip");
        assert_eq!(path, expected_path);
        assert!(std::fs::read_dir("./cache_test/pbkit")
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .fold(false, |acc, entry| acc
                || entry
                    .to_string_lossy()
                    .contains("pbkit/interface-pingpong-server@main")));

        // clean
        // std::fs::remove_file("./cache_test/pbkit/interface-pingpong-server@main.zip").unwrap();
    }
}
