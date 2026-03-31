use std::path::PathBuf;
use pathsearch::find_executable_in_path;

pub fn find_executable(cmd: &str) -> Option<PathBuf> {
    find_executable_in_path(cmd)
}