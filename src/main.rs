// Copyright 2023 Björn Busse
// SPDX-License-Identifier: BSD-3-Clause

use std::error::Error;
use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::process;

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn in_path_and_executable(cmd: &str) -> bool {
    match Command::new(cmd).args(["-v"])
                           .stdout(Stdio::null())
                           .spawn() {
        Ok(_) => true,
        Err(_error) => {
            println!("Failed to execute `git`. Is it installed?");
            process::exit(0x0100);
        },
    };
    false
}

pub fn find_repositories(p: &std::path::PathBuf) -> Vec<String> {
    let binding = fs::canonicalize(p).unwrap();
    let abs_path = binding.to_str().unwrap();
    let mut paths = Vec::new();

    for entry in fs::read_dir(p).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() && path.file_name().unwrap().to_str().unwrap() == ".git" {
            paths.push(abs_path.to_owned())
        } else if path.is_dir() {
            paths.append(&mut find_repositories(&path))
        }
    }
    return paths
}

pub fn working_tree_status(p: String) -> Vec<String> {
    let binding = fs::canonicalize(p.clone()).unwrap();
    let abs_path = binding.to_str().unwrap();
    let path = Path::new(&p);
    let mut paths = Vec::new();
    assert!(env::set_current_dir(path).is_ok());

    let output = Command::new("git")
                             .args(["status"])
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines = stdout.lines();

    if stdout.contains("working tree clean") {
        return paths;
    }

    paths.push(abs_path.to_owned());

    println!("\x1b[93m{}\x1b[0m\n", p);

    for line in lines {
        println!("{}", line)
    }

    return paths;
}

fn main() -> Result<(), Box<dyn Error>>  {
    let args: Vec<String> = env::args().collect();
    let mut p = env::current_dir().unwrap();
    let deps = vec!["git"];
    let mut repos_with_changes = Vec::new();

    for dep in deps {
        in_path_and_executable(dep);
    }

    if ! args[0].is_empty() && path_exists(&args[0]) {
        p = PathBuf::from(&args[0]);
    }

    let repos = find_repositories(&p);

    // Run `git status` on the repos found
    for repo in repos.iter() {
        repos_with_changes.append(&mut working_tree_status(repo.to_string()));
    }

    let nrepos = repos.len();
    let nrepos_with_changes = repos_with_changes.len();

    // Show summary
    if nrepos_with_changes == 0 {
        println!("\x1b[92mFound no (out of {}) repositories with uncommited changes\x1b[0m", nrepos);
    } else if nrepos_with_changes == 1 {
        println!("\n\x1b[93mFound {} (out of {}) repository with uncommited changes\x1b[0m", nrepos_with_changes, nrepos);
    } else if nrepos_with_changes > 1 {
        println!("\n\x1b[93mFound {} (out of {}) repositories with uncommited changes\x1b[0m", nrepos_with_changes, nrepos);
    }

    Ok(())
}
