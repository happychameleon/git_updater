use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};
use std::str;
use std::borrow::Borrow;

//The mvp idea is to simple create a programm that goes in to each folder
// and calls git pull to update the project to the latest state.

fn main() {
    let file_path = "./";

    let paths = fs::read_dir(file_path).unwrap();
    println!("git updater will start updating all git repos in {}", file_path);

    for path in paths {

        let path_project = path.unwrap().path().as_os_str().to_str().unwrap().to_string();
        
        //arguments for git command
        let git_args = vec! ["-C",&path_project[..],"pull"];

        let output = Command::new("git")
            .args(git_args)
            .output()
            .expect("failed to execute process");

        let hello = output.stdout;

        let s = match str::from_utf8(&hello) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("{}", path_project);
        println!("result: {}", s);
    }
}
