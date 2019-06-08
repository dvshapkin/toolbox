//! Virtual file system allows you to work with relative file paths in a convenient way.

mod errors;

use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf, Component};

use errors::{NotAbsolutePathError, NotRelativePathError, PathNotBelongsError};

/// A reference to an virtual file system.
pub struct VirtualFileSystem {
    pub root: PathBuf,
}

impl VirtualFileSystem {

    /// Create new `VirtualFileSystem`.
    ///
    /// `root` - base directory for VFS.
    /// A `root` path must exists; if not, return value will be `io::Error`.
    /// If it's a relative path, then it will be normalized.
    pub fn try_new<P: AsRef<Path>>(root: P) -> Result<Self> {
        Path::new(root.as_ref())
            .canonicalize()
            // normalize ?
            .map(|path| Self { root: path })
    }

    /// Change current `root`.
    ///
    /// A `new_root` path may be absolute or relative and it must exists.
//    pub fn chroot<P: AsRef<Path>>(&mut self, new_root: P) -> Result<()> {
//        self.root = if new_root.as_ref().is_absolute() {
//            new_root.as_ref().canonicalize()?
//        } else {
//            self.absolute(new_root)?
//        };
//        Ok(())
//    }

    /// Convert relative `path` to absolute.
    ///
    /// If `path` is absolute and starts with current `root`, then return it.
    /// If `path` is relative, then append it to the end of the current `root` and return joined path.
    /// If joined path in last case is not exists, then `io::Error` will occure.
//    pub fn absolute<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
//        if path.as_ref().is_relative() {
//            self.root.join(path.as_ref())
//        }
//        //let pb =
//            if path.as_ref().is_absolute() {
//            if self.contains(path.as_ref()) {
//                path.as_ref()
//                    .to_path_buf()
//
//            } else {
//                None
//            }
//        } else {
//            self.root
//                .join(path.as_ref())
//
//        }
//                .canonicalize()
//            .ok()
//    }

    /// Convert absolute `path` to relative.
    ///
    /// If `path` is not absolute, then return `io::Error`.
    /// If `path` is equal to `root`, then return `.` (current).
    /// If `root` equals to `/foo/bar` and `path` equals to `/foo/bar/more`, then return `more`.
//    pub fn relative<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
//        let path = path.as_ref();
//        if path.is_absolute() {
//            if path == self.root {
//                return Ok(PathBuf::from("."));
//            } else {
//                if self.contains(path) {
//                    return Ok(path.strip_prefix(&self.root).unwrap().to_path_buf());
//                }
//            }
//        }
//        Err(Error::new(ErrorKind::Other, NotAbsolutePathError::new()))
//    }



    /// Creates a new, empty directory at the provided path.
    ///
//    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
//        if !self.contains(path.as_ref()) {
//            return Err(Error::new(
//                ErrorKind::Other,
//                PathNotBelongsError::new(path.as_ref()),
//            ));
//        }
//        if path.as_ref().is_absolute() {
//            fs::create_dir(self.root.join(self.relative(path)?))
//        } else {
//            fs::create_dir(self.root.join(path.as_ref()))
//        }
//    }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
//    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
//        if !self.contains(path.as_ref()) {
//            return Err(Error::new(
//                ErrorKind::Other,
//                PathNotBelongsError::new(path.as_ref()),
//            ));
//        }
//        if path.as_ref().is_absolute() {
//            fs::create_dir_all(self.root.join(self.relative(path)?))
//        } else {
//            fs::create_dir_all(self.root.join(path.as_ref()))
//        }
//    }

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    ///
//    pub fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
//        if !self.contains(path.as_ref()) {
//            return Err(Error::new(
//                ErrorKind::Other,
//                PathNotBelongsError::new(path.as_ref()),
//            ));
//        }
//        if self.exists(path.as_ref()) {
//            if path.as_ref().is_absolute() {
//                fs::remove_dir_all(path)?;
//            } else {
//                fs::remove_dir_all(self.root.join(path.as_ref()))?;
//            }
//        }
//        Ok(())
//    }

    /// Verifies, that the `path` belongs to the virtual file system.
    ///
//    fn contains<P: AsRef<Path>>(&self, path: P) -> bool {
//        if path.as_ref().is_absolute() {
//            path.as_ref().starts_with(&self.root)
//        } else {
//            true
//        }
//    }

    fn normalize<P: AsRef<Path>>(&self, path: P) -> PathBuf {

        // TODO: error, if path is empty

        let mut normalized = PathBuf::new();
        match path.as_ref().components().nth(0).unwrap() {
            Component::CurDir => {
                normalized.push(&self.root);
                if path.as_ref().components().count() > 1 {
                    normalized.push(path.as_ref().strip_prefix(Component::CurDir).unwrap())
                }
            },
            Component::ParentDir => {
                if self.root.components().count() == 1 {
                    normalized.push(&self.root)
                } else {
                    normalized.push(&self.root.parent().unwrap())
                }
                if path.as_ref().components().count() > 1 {
                    normalized.push(path.as_ref().strip_prefix(Component::ParentDir).unwrap())
                }
            },
            _ => ()
        };
        for component in path.as_ref().components() {

        }
//        if path.as_ref().starts_with(".") {
//            normalized.push(&self.root);
//            if path.as_ref().components().count() > 1 {
//                normalized.push(path.as_ref().strip_prefix(".").unwrap())
//            }
//        };
//        if path.as_ref().starts_with("..") {
//            if path.as_ref().components().count() == 1 {
//                if self.root.components().count() == 1 {
//                    PathBuf::from(&self.root)
//                } else {
//                    PathBuf::from(&self.root.parent().unwrap())
//                }
//            } else {
//                if &self.root.components().count() == 1 {
//                    PathBuf::from(&self.root)
//                } else {
//                    PathBuf::from(&self.root.parent().unwrap())
//                }
//            }
//        };
        normalized
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    const ROOT: &str = "tests/root";

    fn new_vfs() -> super::VirtualFileSystem {
        super::VirtualFileSystem::try_new(ROOT).unwrap()
    }

    fn cur_dir() -> PathBuf {
        Path::new(ROOT).canonicalize().unwrap()
    }

    #[test]
    fn root_ok() {
        let vfs = new_vfs();
        assert_eq!(vfs.root, cur_dir());
    }

//    #[test]
//    fn chroot_ok() {
//        let mut vfs = new_vfs();
//
//        // new root == old root
//        vfs.chroot(".").unwrap();
//        assert_eq!(vfs.root, cur_dir());
//
//        // new root relative && exists
//        vfs.chroot("more").unwrap();
//        assert_eq!(vfs.root, cur_dir().join("more"));
//
//        // new root absolute && exists
//        vfs.chroot("../..").unwrap();
//        assert_eq!(vfs.root, cur_dir().parent().unwrap());
//    }

//    #[test]
//    #[should_panic(expected = "canonicalize error")]
//    fn chroot_err() {
//        let mut vfs = new_vfs();
//
//        // new root not exists
//        vfs.chroot("more/not_exists").expect("canonicalize error");
//    }

//    #[test]
//    fn absolute_ok() {
//        let vfs = new_vfs();
//        assert_eq!(vfs.absolute("more").unwrap(), cur_dir().join("more"));
//    }

//    #[test]
//    fn relative_ok() {
//        let vfs = new_vfs();
//
//        assert_eq!(
//            vfs.relative(cur_dir().join("more")).unwrap(),
//            Path::new("more")
//        );
//    }

//    #[test]
//    #[should_panic(expected = "Argument is not absolute path.")]
//    fn relative_err() {
//        let vfs = new_vfs();
//
//        vfs.relative("more")
//            .expect("Argument is not absolute path.");
//    }

//    #[test]
//    fn exists_ok() {
//        let vfs = new_vfs();
//        assert!(vfs.exists("more/example.txt"));
//        assert!(!vfs.exists("foo"));
//    }

//    #[test]
//    fn create_dir_ok() {
//        let vfs = new_vfs();
//        vfs.create_dir("new_dir").unwrap();
//    }

//    #[test]
//    #[should_panic(expected = "too many dirs")]
//    fn create_dir_err() {
//        let vfs = new_vfs();
//        vfs.create_dir("new1/new2").expect("too many dirs");
//    }

//    #[test]
//    fn create_dir_all_ok() {
//        let vfs = new_vfs();
//        vfs.create_dir_all("new1/new2").unwrap();
//    }

    //    #[test]
    //    fn remove_dir_all_ok() {
    //        let vfs = new_vfs();
    //        vfs.remove_dir_all("new_dir").unwrap();
    //    }
}
