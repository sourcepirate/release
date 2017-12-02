
extern crate git2;
extern crate regex;

mod version;
mod tags;

use std::env;
use std::path::PathBuf;
use version::SemType;
use tags::Release;

const USAGE : &str = "USAGE: release {major, minor, patch}";

fn main(){
    let args: Vec<String> = env::args().collect();
    let cwd: PathBuf = env::current_dir().unwrap();
    if args.len() <= 1 {
        println!("{}", USAGE);
    }
    else {
        match Release::repo(cwd){
            Ok(repo) => {
                let mut release :Release = repo.release();
                let vtype: SemType = SemType::select(args[1].clone());
                release.incr(vtype);
            },
            Err(E) => {
                println!("Cannot open repo")
            }
        }

    }
}