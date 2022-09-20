use std::path::{Path, PathBuf};

fn get_parent_by_depth<P: AsRef<Path>>(filename: P, depth: usize) -> Vec<PathBuf> {
    let parent = filename.as_ref().parent();
    match parent {
        Some(_) => todo!(),
        None => vec![],
    }
}

pub fn strip<P: AsRef<Path>>(filename: P, depth: usize) -> PathBuf {
    let parent_to_strip = get_parent_by_depth(filename, depth);
    let strip_result = filename.split("/").collect::<Vec<&str>>()[0..depth].join("/");
    if strip_result == "" || strip_result == "/" {
        "".to_string()
    } else {
        strip_result
    }

    return PathBuf::new();
}
