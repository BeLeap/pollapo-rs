use clap::Parser;
use futures::future::join_all;
use piz::read::*;

use crate::{pollapo_yml::{PollapoYml, parse_dep, load_pollapo_yml}, utils::archive::strip};

#[derive(Parser, Debug)]
#[clap(about = "Install dependencies")]
pub struct InstallArgs {
    #[clap(short, long, default_value = "pollapo.yml")]
    pub config: String,

    #[clap(short, long)]
    pub token: String,

    #[clap(short, long, default_value = ".pollapo")]
    pub outdir: String,

    #[clap(short, long, default_value = "~/.config/pollapo/cache")]
    pub cache_dir: String,
}

pub async fn action(config: &str, token: &str, outdir: &str, cache_dir: &str) {
    println!("config: {:?}", config);
    println!("token: {:?}", token);
    println!("outdir: {:?}", outdir);

    let pollapo_yml = load_pollapo_yml(config);
    let cache_dir_futures = pollapo_yml.deps.iter()
        .map(|dep| install_dep_to_cache(&pollapo_yml, &dep, &cache_dir));
    let cache_dirs = join_all(cache_dir_futures).await;
    let recursive_deps = cache_dirs.iter().map(|cache_dir| {
        let zipball_file = std::fs::read(cache_dir).unwrap_or_else(|err| {
            panic!("Failed to open {}: {}", cache_dir.to_string_lossy(), err);
        });
        let zipball = piz::ZipArchive::new(&zipball_file).unwrap_or_else(|err| {
            panic!("Malformed zipball {}: {}", cache_dir.to_string_lossy(), err);
        });
        let zip_tree = as_tree(zipball.entries()).unwrap_or_else(|err| {
            panic!(
                "Malfored zipball converting {}: {}",
                cache_dir.to_string_lossy(),
                err
            );
        });
    });

    println!("{:?}", recursive_deps);

    return {};
}

fn check_cache_hit(
    dep: &str,
    cache_dir: &str,
) {}

async fn install_dep_to_cache(
    pollapo_yml: &PollapoYml,
    dep: &str,
    cache_dir: &str,
) -> std::path::PathBuf {
    let target_hash = &pollapo_yml.root.lock[dep];
    let (user, repo, target_ref) = parse_dep(dep);

    let cache_dir = shellexpand::full(cache_dir).unwrap_or_else(|err| {
        panic!("Failed to resolve {}: {}", cache_dir, err);
    });
    let target_cache_dir = format!("{}/{}", cache_dir, user);
    let target_ref_file_path = format!("{}/{}@{}.{}", target_cache_dir, repo, target_ref, "zip");

    if std::path::Path::new(&target_ref_file_path).exists() {
        std::path::PathBuf::from(target_ref_file_path)
    } else {
        // Fetch zipball
        let zipball_url = format!("https://github.com/{}/{}/zipball/{}", user, repo, target_hash);
        let response = reqwest::get(&zipball_url).await.unwrap_or_else(|err| {
            panic!("Failed to fetch {}: {}", &zipball_url, err);
        });

        // Create file
        let mut content = std::io::Cursor::new(response.bytes().await.unwrap_or_else(|err| {
            panic!("Failed to convert {} into bytes: {}", &zipball_url, err);
        }));

        let file_name = format!("{}@{}.{}", repo, target_hash, "zip");
        let file_name_str = format!("{}/{}", target_cache_dir, file_name);
        let file_path = std::path::Path::new(&file_name_str);
        let file_parent = file_path.parent().unwrap();
        std::fs::create_dir_all(&file_parent).unwrap_or_else(|err| {
            panic!(
                "Failed to create parent {} of {}: {}",
                &file_parent.to_string_lossy(),
                &file_name_str,
                err
            );
        });
        let mut target_file = std::fs::File::create(&file_name_str).unwrap_or_else(|err| {
            panic!("Failed to create {}: {}", &file_name_str, err);
        });

        std::io::copy(&mut content, &mut target_file).unwrap_or_else(|err| {
            panic!("Failed to copy {} to {}: {}", zipball_url, file_name_str, err);
        });

        // Create symbolic link
        std::os::unix::fs::symlink(&file_name, &target_ref_file_path).unwrap_or_else(|err| {
            panic!("Failed to create symbolic link {} to {}: {}", &file_path.to_string_lossy(), &target_ref_file_path, err);
        });

        file_path.to_path_buf()
    }
}

fn extract_cache(path: &std::path::PathBuf, dep: &str, target_dir: Option<&str>) {
    let target_dir_raw = match target_dir {
        Some(dir) => dir,
        None => "~/.config/pollapo/cache",
    };
    let target_dir_str = shellexpand::full(target_dir_raw).unwrap_or_else(|err| {
        panic!("Failed to resolve {}: {}", target_dir_raw, err);
    });
    let target_dir = std::path::Path::new(&*target_dir_str);
    let repo_name = dep.split("@").collect::<Vec<&str>>()[0];
    let target_dir = target_dir.join(repo_name);

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
        let full_path = target_dir.join(strip(&*entry.path, 2));

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
    use serial_test::serial;

    use super::install_dep_to_cache;
    use crate::{cmds::install::extract_cache, pollapo_yml::load_pollapo_yml};

    #[test]
    #[serial]
    fn extract_cache_should_extract_from_cache_to_target() {
        // given
        let pollapo_yml = load_pollapo_yml("pollapo.test.yml");
        let path = tokio_test::block_on(install_dep_to_cache(
            &pollapo_yml,
            "pbkit/interface-pingpong-server@main",
            Some("cache_test"),
        ));

        // when
        extract_cache(
            &path,
            "pbkit/interface-pingpong-server@main",
            Some(".pollapo"),
        );

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
    #[serial]
    fn install_dep_to_cache_should_store_dep_zip_to_cache() {
        // given
        let pollapo_yml = load_pollapo_yml("pollapo.test.yml");

        // when
        tokio_test::block_on(install_dep_to_cache(
            &pollapo_yml,
            "pbkit/interface-pingpong-server@main",
            "cache_test",
        ));

        // then
        assert!(std::fs::read_dir("./cache_test/pbkit")
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .fold(false, |acc, entry| acc
                || entry
                    .to_string_lossy()
                    .contains("pbkit/interface-pingpong-server")));

        // clean
        // std::fs::remove_file("./cache_test/pbkit/interface-pingpong-server@main.zip").unwrap();
    }
}
