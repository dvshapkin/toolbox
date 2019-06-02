//! Virtual file system allows you to work with relative file paths in a convenient way.

mod errors;

use std::path::{Path, PathBuf};
use std::io::{Result, Error, ErrorKind};
use std::fs;

use errors::NotAbsolutePathError;


/// Create new `VirtualFileSystem`.
///
/// `root` - base directory for VFS.
/// A `root` path must exists; if not, return value will be `None`.
/// If it's a relative path, then it will be normalized.
pub fn new<P: AsRef<Path>>(root: P) -> Option<VirtualFileSystem> {
    match Path::new(root.as_ref()).canonicalize() {
        Ok(path) => Some(VirtualFileSystem { root: path }),
        _ => None
    }
}

/// A reference to an virtual file system.
pub struct VirtualFileSystem {
    pub root: PathBuf
}

impl VirtualFileSystem {

    /// Change current `root`.
    ///
    /// A `new_root` path may be absolute or relative and it must exists.
    pub fn chroot<P: AsRef<Path>>(&mut self, new_root: P) -> Result<()> {
        self.root = self.absolute(new_root)?;
        Ok(())
    }

    /// Convert relative `path` to absolute.
    ///
    /// If `path` is absolute, then return it.
    /// If `path` is relative, then append it to the end of the current `root` and return joined path.
    /// If joined path in last case is not exists, then `io::Error` will occure.
    pub fn absolute<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        (if path.as_ref().is_absolute() {
            path.as_ref().to_path_buf()
        } else {
            self.root.join(path)
        }).canonicalize()
    }

    /// Convert absolute `path` to relative.
    ///
    /// If `path` is not absolute, then return `io::Error`.
    /// If `path` is equal to `root`, then return `.` (current).
    /// If `root` equals to `/foo/bar` and `path` equals to `/foo/bar/more`, then return `more`.
    pub fn relative<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        let path = path.as_ref();
        if path.is_absolute() {
            if path == self.root {
                return Ok(PathBuf::from("."));
            } else {
                if path.starts_with(&self.root) {
                    return Ok(path.strip_prefix(&self.root).unwrap().to_path_buf());
                }
            }
        }
        Err(Error::new(ErrorKind::Other, NotAbsolutePathError::from("Argument is not absolute path.")))
    }

    /// Checks the existence of a `path`.
    ///
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        match self.absolute(path) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    /// Creates a new, empty directory at the provided path.
    ///
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if path.as_ref().is_absolute() {
            fs::create_dir(self.root.join(self.relative(path)?))
        } else {
            fs::create_dir(self.root.join(path))
        }
    }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if path.as_ref().is_absolute() {
            fs::create_dir_all(self.root.join(self.relative(path)?))
        } else {
            fs::create_dir_all(self.root.join(path))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    const ROOT: &str = "tests/root";

    fn new_vfs() -> super::VirtualFileSystem {
        super::new(ROOT).unwrap()
    }

    fn cur_dir() -> PathBuf {
        Path::new(ROOT).canonicalize().unwrap()
    }

    #[test]
    fn root_ok() {
        let vfs = new_vfs();
        assert_eq!(vfs.root, cur_dir());
    }

    #[test]
    fn chroot_ok() {
        let mut vfs = new_vfs();

        // new root == old root
        vfs.chroot(".").unwrap();
        assert_eq!(vfs.root, cur_dir());

        // new root relative && exists
        vfs.chroot("more").unwrap();
        assert_eq!(vfs.root, cur_dir().join("more"));

        // new root absolute && exists
        vfs.chroot("../..").unwrap();
        assert_eq!(vfs.root, cur_dir().parent().unwrap());
    }

    #[test]
    #[should_panic(expected="canonicalize error")]
    fn chroot_err() {
        let mut vfs = new_vfs();

        // new root not exists
        vfs.chroot("more/not_exists").expect("canonicalize error");
    }

    #[test]
    fn absolute_ok() {
        let vfs = new_vfs();
        assert_eq!(vfs.absolute("more").unwrap(), cur_dir().join("more"));
    }

    #[test]
    fn relative_ok() {
        let vfs = new_vfs();

        assert_eq!(vfs.relative(cur_dir().join("more")).unwrap(), Path::new("more"));
    }

    #[test]
    #[should_panic(expected="Argument is not absolute path.")]
    fn relative_err() {
        let vfs = new_vfs();

        vfs.relative("more").expect("Argument is not absolute path.");
    }

    #[test]
    fn exists_ok() {
        let vfs = new_vfs();
        assert!(vfs.exists("more/example.txt"));
        assert!(! vfs.exists("foo"));
    }

    #[test]
    fn create_dir_ok() {
        let vfs = new_vfs();
        vfs.create_dir("new_dir").unwrap();
    }

    #[test]
    #[should_panic(expected="too many dirs")]
    fn create_dir_err() {
        let vfs = new_vfs();
        vfs.create_dir("new1/new2").expect("too many dirs");
    }

    #[test]
    fn create_dir_all_ok() {
        let vfs = new_vfs();
        vfs.create_dir_all("new1/new2").unwrap();
    }
}
