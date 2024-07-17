use crate::utils::scan_files_in_dir;
use chrono::{Datelike,  Utc};
use std::{
    collections::HashMap, fmt::format, fs, hash::DefaultHasher, path::{Path, PathBuf}
};
use std::hash::{Hash, Hasher};

const MAX_NUM_LEN: usize = 99;

fn check_exist(name: &String, v:&str) -> bool {
    name.contains(v)
}

fn check_num_len(name: &String) -> usize {
    let numflag = name.contains("{num");
    let mut numlen: usize = 0;
    if !numflag {
        return numlen;
    }
    if name.contains("{num}") {
        return MAX_NUM_LEN;
    }
    if !name.contains("{num:") {
        return 0;
    }
    numlen = MAX_NUM_LEN;
    match name.find("{num:") {
        Some(v) => {
            let nn = &name[v + 5..];
            let vv = nn.find("}").unwrap();
            let nnn = &nn.to_string()[..vv];
            numlen = 0;
            for c in nnn.chars() {
                if !c.is_ascii_digit() {
                    if c == 'd' || c == 'f' {
                        break;
                    }
                    return 0;
                }
                numlen = numlen * 10 + c.to_digit(10).unwrap() as usize;
            }
            if numlen == 0 {
                numlen = MAX_NUM_LEN;
            }
        }
        None => {}
    }
    numlen
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_get_num() {
        assert_eq!(check_num_len(&"aaa{num}".to_string()), 99);
        assert_eq!(check_num_len(&"aaa{num:2}".to_string()), 2);
        assert_eq!(check_num_len(&"aaa{num:03d}".to_string()), 3);
        assert_eq!(check_num_len(&"aaa{num:03f}".to_string()), 3);
        assert_eq!(check_num_len(&"aaa{numasda".to_string()), 0);
    }
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

fn do_rename(mut rmaps: HashMap<&PathBuf, String>) {
    let mut rmkeys:Vec<&PathBuf> = Vec::new();
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
                    Ok(_v) => {
                    }   
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            break;
        }
        for k in rmkeys.iter() {
            rmaps.remove(k);
        }
    }
}

#[allow(dead_code)]
pub fn g_rename(idir: &String, suffix: &String, name:String, r:bool, p:bool) -> bool {
    println!("Ready to rename {suffix} files in {idir}, ==> {name}, with flag, r({r}), p({p})");
    let files = scan_files_in_dir(&idir, r);
    if files.len() == 0 {
        return false;
    }
    let todos: Vec<&PathBuf> = files.iter().filter(| x | x.as_path().ends_with(suffix)).collect();
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
    let numlen = check_num_len(&lowername);
    let mut start: usize = 1;
    let mut domap: HashMap<&PathBuf, String> = HashMap::with_capacity(todos.len());
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
            npath = npath.replace("{num}", &num);
            start += 1;
        }
        domap.insert(v, npath);
    }
    // check is repeat
    if domap.len() > 0 {
        let mut tlist:Vec<&String> = Vec::new();
        for (_k, v) in domap.iter() {
            tlist.push(v);
        }
        tlist.sort();
        for i in 0..tlist.len() -1 {
            if tlist[i].eq(tlist[i+1]) {
                return false;
            }
        }
    }
    // preview only
    if p {
        for (k, v) in domap.iter() {
            println!("{} ===> {}", k.to_str().unwrap(), v);
        }
        return true;
    }
    do_rename(domap);
    return true;
}

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
    let numlen = check_num_len(&lname);
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
            if numlen > 0 {
                let mut num = start.to_string();
                if numlen > num.len() {
                    num = "0".repeat(numlen - num.len()) + &num;
                }
                npath = npath.replace("{num}", &num);
                start += 1;
            }
            fs::rename(v, npath).unwrap();
        }
    }

    return true;
}
