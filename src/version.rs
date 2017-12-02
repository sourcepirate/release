
use std::fmt;

pub enum SemType {
    MAJOR,
    MINOR,
    PATCH
}

impl SemType {
    pub fn incr_version(&self, version: Version) -> Version {
      match *self {
          SemType::MAJOR => Version::new(version.major+1, version.minor, version.patch),
          SemType::MINOR => Version::new(version.major, version.minor + 1, version.patch),
          SemType::PATCH => Version::new(version.major, version.minor, version.patch + 1)
      }
    }

    pub fn select(flag: String) -> Self {
        match flag.as_ref() {
            "major" => SemType::MAJOR,
            "minor" => SemType::MINOR,
            "patch" => SemType::PATCH,
             _ => SemType::PATCH
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version{

    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version {
            major,
            minor,
            patch
        }
    }

    pub fn incr(&self, semtype: SemType) -> Self {
        semtype.incr_version(*self)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_version_string(){
        let version: Version = Version::new(1, 0,0);
        assert_eq!(format!("{}", version), "1.0.0")
    }

    #[test]
    fn test_version_debug_string(){
        let version: Version = Version::new(1,0,0);
        assert_eq!(format!("{:?}", version), "v1.0.0")
    }

    #[test]
    fn test_increment_version(){
        let version: Version = Version::new(1,0,0);
        let incrv : Version = version.incr(SemType::PATCH);
        assert_eq!(format!("{}", incrv), "1.0.1")
    }

}