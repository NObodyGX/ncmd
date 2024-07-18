use crate::utils::scan_files_in_dir;
use ansi_term::Colour::{Green, Red};
use chrono::{Datelike, Utc};
use indexmap::IndexMap;
use std::cmp::{max, min};
use std::hash::{Hash, Hasher};
use std::{fs, hash::DefaultHasher, path::PathBuf};

const MAX_NUM_LEN: usize = 99;

fn check_exist(name: &String, v: &str) -> bool {
    name.contains(v)
}

fn get_num_str(name: &String) -> String {
    if name.contains("{num}") {
        return "{num}".to_string();
    }
    if !name.contains("{num:") {
        return "".to_string();
    }
    let sn = name.find("{num:").unwrap();
    let aftername = &name[sn..];
    match aftername.find("}") {
        Some(v) => {
            return name[sn..sn + v+1].to_string();
        }
        None => {
            return name[sn..].to_string() + "}";
        }
    }
}

fn check_num(name: &String) -> (String, usize) {
    let numhold = get_num_str(name);
    let pv = "{num:".len();
    if numhold == "{num}" {
        return (numhold, MAX_NUM_LEN);
    }
    if numhold.len() <= pv {
        return (numhold, 0);
    }
    if !numhold.contains("}") {
        return (numhold, 0);
    }
    let dv = numhold.find("}").unwrap();
    let numstr = &numhold[pv..dv];

    let mut numlen = 0;
    for c in numstr.chars() {
        if !c.is_ascii_digit() {
            if c == 'd' || c == 'f' {
                break;
            }
            return ("".to_string(), 0);
        }
        numlen = numlen * 10 + c.to_digit(10).unwrap() as usize;
    }

    (numhold, numlen)
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_get_num() {
        assert_eq!(check_num(&"aaa{num}".to_string()), ("{num}".to_string(), 99));
        assert_eq!(check_num(&"aaa{num:2}".to_string()), ("{num:2}".to_string(), 2));
        assert_eq!(check_num(&"aaa{num:03d}".to_string()), ("{num:03d}".to_string(), 3));
        assert_eq!(check_num(&"aaa{num:03f}".to_string()), ("{num:03f}".to_string(), 3));
        assert_eq!(check_num(&"aaa{numasda".to_string()), ("".to_string(), 0));
    }

    #[test]
    fn test_path() {
        let mut files: Vec<PathBuf> = Vec::new();
        files.push(PathBuf::from("aaa.txt"));
        files.push(PathBuf::from("bbb.txt"));
        files.push(PathBuf::from("ccc.txt"));
        files.push(PathBuf::from("ddd.txt1"));
        files.push(PathBuf::from("eee.TxT"));
        let todos: Vec<&PathBuf> = files
            .iter()
            .filter(|x| match x.extension() {
                Some(v) => v.to_ascii_lowercase().eq("txt"),
                None => false,
            })
            .collect();
        println!("len: {}", todos.len());
    }

    #[test]
    fn test_color() {
        let v: String = String::from("askdlajd");
        println!("{} ===> {}", "aaa", Red.paint(&v));
    }
}

fn check_print_repeat(rmaps: &IndexMap<&PathBuf, String>) -> bool {
    if rmaps.len() == 0 {
        return true;
    }

    let mut mkeys: Vec<&String> = Vec::new();
    let mut mrkey: Vec<&String> = Vec::new();

    for (_k, v) in rmaps.iter() {
        if mkeys.contains(&v) {
            mrkey.push(v);
            continue;
        }
        mkeys.push(v);
    }
    if mrkey.len() > 0 {
        for (k, v) in rmaps.iter() {
            if mrkey.contains(&v) {
                println!("{} ===> {}", k.to_str().unwrap(), Red.paint(v));
                continue;
            }
            println!("{} ===> {}", k.to_str().unwrap(), Green.paint(v));
        }
        return false;
    }
    return true;
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn get_hash_name(name: &String) -> String {
    let hashd = calculate_hash(name);
    format!("{}_{}", name, hashd)
}

fn do_rename(mut rmaps: IndexMap<&PathBuf, String>) {
    let mut rmkeys: Vec<&PathBuf> = Vec::new();
    loop {
        rmkeys.clear();
        for (k, v) in rmaps.iter() {
            let pbuf = PathBuf::from(v);
            if pbuf.exists() {
                continue;
            }
            match fs::rename(k, pbuf) {
                Ok(_v) => {
                    rmkeys.push(k);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        if rmkeys.len() == rmaps.len() {
            break;
        }
        if rmkeys.len() == 0 && rmaps.len() != 0 {
            for (k, v) in rmaps.iter() {
                match fs::rename(k, get_hash_name(v)) {
                    Ok(_v) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            for (_k, v) in rmaps.iter() {
                match fs::rename(get_hash_name(v), v) {
                    Ok(_v) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            break;
        }
        for k in rmkeys.iter() {
            rmaps.shift_remove(k);
        }
    }
}

#[allow(dead_code)]
pub fn g_rename(idir: &String, suffix: &String, name: String, r: bool, p: bool) -> bool {
    println!("Ready to rename {suffix} files in {idir}, ==> {name}, with flag, r({r}), p({p})");
    let files = scan_files_in_dir(&idir, r);
    if files.len() == 0 {
        return false;
    }
    let todos: Vec<&PathBuf> = files
        .iter()
        .filter(|x| match x.extension() {
            Some(v) => v.to_ascii_lowercase().eq(suffix.as_str()),
            None => false,
        })
        .collect();
    let now = Utc::now();

    let lowername = name.to_lowercase();
    let cdate = format!("{}-{:02}-{:02}", now.year(), now.month(), now.day());
    let timeflag = check_exist(&lowername, "{time}");
    let dateflag = check_exist(&lowername, "{date}");
    let nname = if dateflag {
        name.replace("{date}", &cdate)
    } else {
        name.clone()
    };
    let (numhold, numlen) = check_num(&lowername);
    let numlen =  if numlen == MAX_NUM_LEN {max(todos.len(), 2)} else {numlen};
    let mut start: usize = 1;
    let mut domap: IndexMap<&PathBuf, String> = IndexMap::with_capacity(todos.len());
    for v in todos.iter() {
        let mut npath = nname.clone();
        if timeflag {
            let now = Utc::now();
            let ntime = format!("{}", now.timestamp_millis());
            npath = npath.replace("{time}", &ntime);
        }
        if numlen > 0 {
            let mut num = start.to_string();
            if numlen > num.len() {
                num = "0".repeat(numlen - num.len()) + &num;
            }
            npath = npath.replace(&numhold, &num);
            start += 1;
        }
        if !npath.ends_with(suffix) {
            npath += ".";
            npath += suffix;
        }
        domap.insert(v, npath);
    }
    // check whether is repeat, print if error
    if !check_print_repeat(&domap) {
        return false;
    }

    // preview only
    if p {
        for (k, v) in domap.iter() {
            println!("{} ===> {}", k.to_str().unwrap(), Green.paint(v));
        }
        return true;
    }
    do_rename(domap);
    return true;
}

#[allow(dead_code)]
pub fn g_renames(idir: &String, isuffixs: &Vec<String>, name: String, r: bool, p: bool) -> bool {
    println!("Ready to rename files in {idir}, ==> {name}, with flag, r({r}), p({p})");

    let mut todolist: Vec<Vec<PathBuf>> = Vec::new();
    for _ in isuffixs.iter() {
        let mlist = Vec::new();
        todolist.push(mlist);
    }

    let path = fs::canonicalize(idir);
    match path {
        Ok(path) => {
            if !path.exists() {
                return false;
            }
            let files = scan_files_in_dir(&idir, r);
            for fbuf in files.iter() {
                for (i, suffix) in isuffixs.iter().enumerate() {
                    if fbuf.as_path().ends_with(suffix) {
                        todolist[i].push(fbuf.clone());
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("{e}\nerror: {idir}.");
        }
    }

    let now = Utc::now();
    let lname = name.to_lowercase();
    let mut nname = name.clone();
    let cdate = format!("{}-{:02}-{:02}", now.year(), now.month(), now.day());
    let timeflag = check_exist(&lname, "{time}");
    let dateflag = check_exist(&lname, "{date}");
    if dateflag {
        nname = name.replace("{date}", &cdate);
    }
    let numlen = check_num(&lname);
    let mut start: usize = 1;
    for (i, _suffix) in isuffixs.iter().enumerate() {
        let mlist = todolist.get(i).unwrap();
        if mlist.len() == 0 {
            continue;
        }
        for v in mlist.iter() {
            let mut npath = nname.clone();
            if timeflag {
                let now = Utc::now();
                let ntime = format!("{}", now.timestamp_millis());
                npath = npath.replace("{time}", &ntime);
            }
            fs::rename(v, npath).unwrap();
        }
    }

    return true;
}
