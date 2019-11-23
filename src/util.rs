use std::path::Path;
use std::{fs, io};

pub fn create_dir_by_ext(p: &Path) -> io::Result<()> {
    fs::create_dir_all(
        if !p.extension().unwrap().is_empty() {
            p.parent().unwrap()
        } else { p }
    )
}