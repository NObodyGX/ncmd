use std::{
    fs,
    path::PathBuf,
};

use walkdir::{DirEntry, WalkDir};

fn is_hidden_unix(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn scan_files_recursion(idir: &String, depth: usize) -> Vec<PathBuf> {
    let mut mlist: Vec<PathBuf> = Vec::new();

    let path = fs::canonicalize(idir);
    match path {
        Ok(path) => {
            if !path.exists() {
                return mlist;
            }
            let walker = WalkDir::new(path).max_depth(depth).into_iter();
            for e in walker.filter_entry(|e| !is_hidden_unix(e)) {
                match e {
                    Ok(entry) => {
                        if !entry.file_type().is_file() {
                            continue;
                        }
                        mlist.push(entry.path().to_path_buf());
                    }
                    Err(err) => {
                        println!("error {}", err);
                    }
                }
            }
        }
        Err(e) => {
            println!("{e}\nerror: {idir}.");
            return mlist;
        }
    }

    mlist
}

pub fn scan_files_in_dir(idir: &String, r: bool) -> Vec<PathBuf> {
    if r {
        return scan_files_recursion(idir, std::usize::MAX);
    }
    return scan_files_recursion(idir, 1);
}
