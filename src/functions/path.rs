use std::path::{Component, Path, PathBuf};

pub fn resolve_path(path: &Path) -> PathBuf {
    let mut stack = Vec::new();

    for component in path.components() {
        match component {
            Component::Normal(name) => stack.push(name.to_str().unwrap().to_string()),
            Component::ParentDir => {
                stack.pop();
            }
            Component::RootDir => stack = vec![String::new()],
            _ => {}
        }
    }

    if stack.len() == 1 && stack[0].is_empty() {
        return PathBuf::from("/");
    }

    PathBuf::from(stack.join("/"))
}
