//! Virtual file system allows you to work with relative file paths in a convenient way.

use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::{Component, Path, PathBuf};

/// A reference to an virtual file system.
pub struct VirtualFileSystem {
    pub root: PathBuf,
}

impl VirtualFileSystem {

    /// Create new `VirtualFileSystem`.
    ///
    /// `root` - base directory for VFS.
    /// A `root` path must exists; else return value will be `io::Error`.
    /// It will be normalized.
    pub fn try_new<P: AsRef<Path>>(root: P) -> Result<Self> {
        Path::new(root.as_ref())
            .canonicalize()
            .map(|path| Self { root: path })
    }

    /// Convert relative `path` to absolute.
    ///
    /// If `path` is absolute and starts with current `root`, then return it, else return `None`.
    /// If `path` is relative, then append it to the end of the current `root` and return joined path.
    pub fn absolute<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        if path.as_ref().is_relative() {
            Some(Self::normalize(self.root.join(path.as_ref())))
        } else {
            let path_norm = Self::normalize(path.as_ref());
            if path_norm.starts_with(&self.root) {
                Some(path_norm)
            } else {
                None
            }
        }
    }

    /// Convert absolute `path` to relative.
    ///
    /// If `path` is relative, then normalize it and return.
    /// If `path` is equal to `root`, then return `.` (current).
    /// If `root` equals to `/foo/bar` and `path` equals to `/foo/bar/more`, then return `more`.
    pub fn relative<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        let path = self.absolute(path)?;
        Some(Self::normalize(path.strip_prefix(&self.root).unwrap()))
    }

    /// Returns true if the path points at an existing entity.
    ///
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(path) = self.absolute(path) {
            path.exists()
        } else {
            false
        }
    }

    /// Change current `root`.
    ///
    /// A `new_root` path may be absolute or relative.
    /// Return true if `root` was change.
    pub fn chroot<P: AsRef<Path>>(&mut self, new_root: P) -> bool {
        match self.absolute(new_root) {
            Some(path) => {
                self.root = path;
                true
            },
            None => false
        }
    }

    /// Creates a new, empty directory at the provided path.
    ///
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self.absolute(path) {
            Some(path) => {
                fs::create_dir(path)?;
                Ok(())
            },
            None => Err(Error::from(ErrorKind::NotFound))
        }
    }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self.absolute(path) {
            Some(path) => {
                fs::create_dir_all(path)?;
                Ok(())
            },
            None => Err(Error::from(ErrorKind::NotFound))
        }
    }

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    ///
    pub fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self.absolute(path) {
            Some(path) => {
                fs::remove_dir_all(path)?;
                Ok(())
            },
            None => Err(Error::from(ErrorKind::NotFound))
        }
    }

    pub fn normalize<P: AsRef<Path>>(path: P) -> PathBuf {
        match path.as_ref().components().count() {
            0 => PathBuf::from("."),

            1 => PathBuf::from(path.as_ref()),

            _ => {
                let mut normalized = PathBuf::new();
                for component in path.as_ref().components() {
                    match component {
                        Component::CurDir => {}
                        Component::ParentDir => {
                            if normalized.components().count() == 0 {
                                normalized.push(component);
                            } else {
                                if normalized.components().last().unwrap() == Component::ParentDir {
                                    normalized.push(component);
                                } else {
                                    normalized.pop();
                                }
                            }
                        }
                        _ => normalized.push(component),
                    }
                }
                normalized
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::VirtualFileSystem;

    const ROOT: &str = "tests/root";

    fn new_vfs() -> VirtualFileSystem {
        VirtualFileSystem::try_new(ROOT).unwrap()
    }

    fn cur_dir() -> PathBuf {
        Path::new(ROOT).canonicalize().unwrap()
    }

    #[test]
    fn normalize_ok() {
        assert_eq!(VirtualFileSystem::normalize(""), PathBuf::from("."));
        assert_eq!(VirtualFileSystem::normalize("."), PathBuf::from("."));
        assert_eq!(VirtualFileSystem::normalize(".."), PathBuf::from(".."));
        assert_eq!(VirtualFileSystem::normalize("../."), PathBuf::from(".."));
        assert_eq!(VirtualFileSystem::normalize("../.."), PathBuf::from("../.."));
        assert_eq!(VirtualFileSystem::normalize("../../.."), PathBuf::from("../../.."));
        assert_eq!(VirtualFileSystem::normalize(".././.."), PathBuf::from("../.."));
        assert_eq!(VirtualFileSystem::normalize("./dir"), PathBuf::from("dir"));
        assert_eq!(VirtualFileSystem::normalize("../dir"), PathBuf::from("../dir"));
        assert_eq!(VirtualFileSystem::normalize("../dir/.."), PathBuf::from(".."));
        assert_eq!(VirtualFileSystem::normalize("./first/second/.."), PathBuf::from("first"));
        assert_eq!(VirtualFileSystem::normalize("first/./second"), PathBuf::from("first/second"));
        assert_eq!(VirtualFileSystem::normalize("\\\\?\\C:\\"), PathBuf::from("\\\\?\\C:\\"));
        assert_eq!(VirtualFileSystem::normalize("\\\\?\\C:/."), PathBuf::from("\\\\?\\C:/"));
    }

    #[test]
    fn root_ok() {
        let vfs = new_vfs();
        assert_eq!(vfs.root, cur_dir());
    }

    #[test]
    fn absolute_ok() {
        let vfs = new_vfs();
        assert_eq!(vfs.absolute(".").unwrap(), cur_dir());
        assert_eq!(vfs.absolute("more").unwrap(), cur_dir().join("more"));
        assert_eq!(vfs.absolute(cur_dir().join("more")).unwrap(), cur_dir().join("more"));
        assert_eq!(vfs.absolute(PathBuf::from("/other/absolute")), None);
    }

    #[test]
    fn relative_ok() {
        let vfs = new_vfs();
        assert_eq!(vfs.relative("./relative").unwrap(), PathBuf::from("relative"));
        assert_eq!(vfs.relative(cur_dir()).unwrap(), PathBuf::from("."));
        assert_eq!(vfs.relative(cur_dir().join("more")).unwrap(), PathBuf::from("more"));
        assert_eq!(vfs.relative(PathBuf::from("/other/absolute")), None);
    }

    #[test]
    fn exists_ok() {
        let vfs = new_vfs();
        assert!(vfs.exists("more/example.txt"));
        assert!(!vfs.exists("foo"));
    }

    #[test]
    fn chroot_ok() {
        let mut vfs = new_vfs();

        // new root == old root
        assert!(vfs.chroot("."));
        assert_eq!(vfs.root, cur_dir());

        // new root relative
        assert!(vfs.chroot("more"));
        assert_eq!(vfs.root, cur_dir().join("more"));

        // new root absolute
        assert!(vfs.chroot("../.."));
        assert_eq!(vfs.root, cur_dir().parent().unwrap());
    }

    #[test]
    fn create_dir_ok() {
        let vfs = new_vfs();
        vfs.create_dir("new_dir").unwrap();
    }

    #[test]
    fn create_dir_all_ok() {
        let vfs = new_vfs();
        vfs.create_dir_all("new1/new2").unwrap();
    }

    #[test]
    fn remove_dir_all_ok() {
        let vfs = new_vfs();
        vfs.remove_dir_all("new_dir").unwrap();
    }
}
