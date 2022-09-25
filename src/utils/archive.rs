use std::path::{Path, PathBuf};

fn get_parent_by_depth<P: AsRef<Path>>(filepath: P, depth: usize) -> PathBuf {
    let ancestor_vec = filepath.as_ref().ancestors().collect::<Vec<&Path>>();
    ancestor_vec[ancestor_vec.len() - depth].to_path_buf()
}

pub fn strip<P: AsRef<Path>>(filepath: P, depth: usize) -> PathBuf {
    let filepath = filepath.as_ref();
    let parent_to_strip = get_parent_by_depth(filepath, depth);

    filepath
        .strip_prefix(
            parent_to_strip
                .iter()
                .fold(PathBuf::new(), |acc, elem| acc.join(elem)),
        )
        .unwrap()
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::utils::archive::get_parent_by_depth;

    #[test]
    fn test_get_parent_by_depth() {
        let filepath = "foo/bar/quuz";
        let parent_vec = get_parent_by_depth(filepath, 3);

        assert_eq!(parent_vec, PathBuf::from("foo/bar"));
    }
}
