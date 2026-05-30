use pathsearch::find_executable_in_path;
use std::path::PathBuf;

pub fn find_executable(cmd: &str) -> Option<PathBuf> {
    find_executable_in_path(cmd)
}
