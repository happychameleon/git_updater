extern crate clap;
use clap::{App, Arg};

use std::fs;
use std::process::{Command};
use std::str;

//The mvp idea is to simple create a program that goes in to each folder
// and calls git pull to update the project to the latest state.
fn main() {
    let matches = App::new("The git updater")
        .version("0.1").author("Max Hackinger")
        .about("this is a simple program that updates all the git repositories laying around on your machine that you want updated")
        .arg(Arg::with_name("directory")
            .short("d")
            .long("directory")
            .value_name("file_path")
            .help("sets the directory path from which the git repos should be updated")
            .required(true))
        .get_matches();

    let file_path = matches.value_of("directory").unwrap();

    println!(
        "git updater will start updating all git repos in {}",
        file_path
    );

    search_dir(file_path);
}

fn search_dir(file_path: &str){
    let paths = fs::read_dir(file_path).unwrap();

    for path in paths {
        let path_project = path
            .as_ref()
            .unwrap()
            .path()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();
        if path.as_ref().unwrap().file_type().unwrap().is_dir() && is_git_repo(&path_project) {
            git_update_command(&path_project);
        }
        else if path.as_ref().unwrap().file_type().unwrap().is_dir() {
            search_dir(path_project.as_str())
        }
    }
}

fn is_git_repo(path_project: &String) -> bool{
    let mut is_repo = false;
    let paths = fs::read_dir(path_project).unwrap();
    for path in paths {
        let new_path = path
            .unwrap()
            .file_name()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();
        if new_path.eq(&".git".to_string()) {
            is_repo = true;
        }
    }
    return is_repo;
}

fn git_update_command(path_project: &String) {
    //arguments for git command
    let git_args = vec!["-C", &path_project[..], "pull"];

    let output = Command::new("git")
        .args(git_args)
        .output()
        .expect("failed to execute process");

    let output = output.stdout;

    let s = match str::from_utf8(&output) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", path_project);
    println!("result: {}", s);
}
