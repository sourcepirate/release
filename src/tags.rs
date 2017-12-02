

use std::path::PathBuf;
use std::env::current_dir;
use std::slice::Iter;
use regex::Regex;
use git2::{Repository, Error, Oid};
use git2::string_array::StringArray;
use version::{Version, SemType};

pub trait Vesionable{
    fn versions(&self) -> Option<Vec<Version>>;
}

pub type GitTags = Result<StringArray, Error>;

const SEMPATTERN : &str = r"(\d+)\.(\d+)\.(\d+)";

impl Vesionable for GitTags {
    fn versions(&self) -> Option<Vec<Version>> {
        let reg : Regex = Regex::new(SEMPATTERN).unwrap();
        match *self {
            Ok(ref arr) => {
                let tagged_versions: Vec<Version> = arr.iter()
                    .filter(|str_version| reg.is_match(str_version.unwrap()))
                    .map(|str_version|{
                        let captures = reg.captures(str_version.unwrap()).unwrap();
                        let major : u32 = captures.get(1).unwrap().as_str().parse().unwrap();
                        let minor : u32 = captures.get(2).unwrap().as_str().parse().unwrap();
                        let patch : u32 = captures.get(3).unwrap().as_str().parse().unwrap();
                        Version::new(major, minor, patch)
                    }).collect();
                Some(tagged_versions)
            },
            Err(_) => None
        }
    }
}

pub struct RepoController(Repository);

impl RepoController{
    pub fn get_tags(&self) -> GitTags {
        self.0.tag_names(Some("*.*.*"))
    }

    pub fn release(&self) -> Release {
        let mut versions = self.get_tags().versions();
        Release::new(&self.0, versions)
    }
}

pub struct Release<'a> {
    repo: &'a Repository,
    versions: Vec<Version>
}

impl<'a> Release<'a> {

    pub fn new(repo: &'a Repository, versions: Option<Vec<Version>>) -> Self {
        let mut current_version = vec![];
        if !versions.is_none(){
            current_version = versions.unwrap();
            current_version.sort();
        }
        Release{
            repo,
            versions:current_version
        }
    }

    pub fn get_initial_version() -> Version {
        Version::new(0,0, 0)
    }

    pub fn repo(p: PathBuf) -> Result<RepoController, Error>{
        let mut repository = try!(Repository::open(p));
        Ok(RepoController(repository))
    }

    pub fn incr(&mut self, vtype: SemType) {
        let mut val;
        if self.versions.is_empty(){
            let initial = Release::get_initial_version();
            val = initial.incr(vtype);
        } else {
            let latest = self.versions.last().unwrap();
            val = latest.incr(vtype);
        }
        self.versions.push(val);
    }

    pub fn make_release(&mut self, vtype: SemType) -> Result<Oid, Error> {
        self.incr(vtype);
        let mut obj;
        let latest = self.versions.last().unwrap();
        {
            obj = try!(self.repo.revparse_single("HEAD"));
        }
        self.repo.tag_lightweight(format!("{}", latest).as_str(), &obj, false)
    }

    pub fn iter(&self) -> Iter<Version>{
        self.versions.iter()
    }
}
