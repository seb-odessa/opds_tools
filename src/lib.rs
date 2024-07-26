use regex::Regex;
use std::collections::LinkedList;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn find_library(root: &str) -> Result<Vec<PathBuf>, io::Error> {
    let library = OsStr::new("lib.rus.ec");
    let mut stack = LinkedList::from([PathBuf::from(root)]);
    let mut subdirs = Vec::new();
    while let Some(current) = stack.pop_back() {
        if let Ok(entries) = fs::read_dir(&current) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if path.file_name() == Some(&library) {
                            subdirs.push(path);
                        } else {
                            stack.push_front(path);
                        }
                    }
                }
            }
        }
    }
    Ok(subdirs)
}

pub fn find_archives(root: &str, id: u32) -> Result<Vec<PathBuf>, io::Error> {
    let mut archives = Vec::new();
    let re = Regex::new(r"fb2-(\d+)-(\d+)(?:_lost)?\.zip").unwrap();

    for entry in fs::read_dir(root)? {
        let path = entry?.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                if let Some(caps) = re.captures(file_name) {
                    let start: u32 = caps[1].parse().unwrap_or_default();
                    let end: u32 = caps[2].parse().unwrap_or_default();

                    if start <= id && id <= end {
                        archives.push(path);
                    }
                }
            }
        }
    }

    Ok(archives)
}
