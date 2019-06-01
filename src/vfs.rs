//! Virtual file system allows you to work with relative file paths in a convenient way

mod errors;

use std::path::{Path, PathBuf};
use std::io::{Result, Error, ErrorKind};
use std::fs;

use errors::NotAbsolutePathError;


/// Create new `VirtualFileSystem`
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

/// A reference to an virtual file system
pub struct VirtualFileSystem {
    pub root: PathBuf
}

impl VirtualFileSystem {

    /// Change current `root`.
    ///
    /// A `new_root` path must exists; it may be absolute or relative.
    pub fn chroot<P: AsRef<Path>>(&mut self, new_root: P) -> Result<()> {
        let new_root = new_root.as_ref();
        match Path::new(new_root).canonicalize() {
            Ok(new_root) => {
                if new_root != self.root {
                    self.root = self.absolute(&new_root);
                }
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    /// Convert relative `path` to absolute.
    ///
    /// If `path` is absolute, then return it.
    /// If `path` is relative, then append it to the end of the current `root` and return.
    pub fn absolute<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        if path.as_ref().is_absolute() {
            path.as_ref().to_path_buf()
        } else {
            self.root.join(path)
        }
    }

    /// Convert absolute `path` to relative
    ///
    /// If `path` is not absolute, then return `io::Error`.
    /// If `path` is equal to `root`, then return `.` (current).
    /// If `root = "/foo/bar"` and `path = "/foo/bar/more"`, then return `more`.
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
        Err(Error::new(ErrorKind::Other, NotAbsolutePathError::from("")))
    }

    /// Creates a new, empty directory at the provided path
    ///
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        if path.is_absolute() {
            fs::create_dir(self.root.join(self.relative(path)?))
        } else {
            fs::create_dir(self.root.join(path))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path};

    #[test]
    fn root_ok() {
        let vfs = super::new(".").unwrap();
        assert_eq!(vfs.root, Path::new("F:\\projects\\rust\\vfs").canonicalize().unwrap());
    }

    #[test]
    fn chroot_ok() {
        let mut vfs = super::new(".").unwrap();

        // new root == old root
        vfs.chroot(".").expect("canonicalize error");
        assert_eq!(vfs.root, Path::new("F:\\projects\\rust\\vfs").canonicalize().unwrap());

        // new root relative && exists
        vfs.chroot("src").expect("canonicalize error");
        assert_eq!(vfs.root, Path::new("F:\\projects\\rust\\vfs\\src").canonicalize().unwrap());

        // new root absolute && exists
        vfs.chroot("F:\\projects").expect("canonicalize error");
        assert_eq!(vfs.root, Path::new("F:\\projects").canonicalize().unwrap());
    }

    #[test]
    #[should_panic(expected="canonicalize error")]
    fn chroot_err() {
        let mut vfs = super::new(".").unwrap();

        // new root not exists
        vfs.chroot("F:\\projects\\foo").expect("canonicalize error");
        assert_eq!(vfs.root, Path::new("F:\\projects\\foo").canonicalize().unwrap());
    }

    #[test]
    fn relative_ok() {
        let vfs = super::new(".").unwrap();
        assert_eq!(vfs.root, Path::new("/home/dvshapkin/projects/rust/toolbox").canonicalize().unwrap());

        assert_eq!(vfs.relative(Path::new("/home/dvshapkin/projects/rust/toolbox")).unwrap(), Path::new("."));
        assert_eq!(vfs.relative(Path::new("/home/dvshapkin/projects/rust/toolbox/Cargo.toml")).unwrap(), Path::new("Cargo.toml"));
    }

    #[test]
    fn create_dir_ok() {
        let vfs = super::new("tests").unwrap();
        vfs.create_dir("new_dir").unwrap();
    }

    #[test]
    #[should_panic(expected="too many dirs")]
    fn create_dir_err() {
        let vfs = super::new("tests").unwrap();
        vfs.create_dir("new1/new2").expect("too many dirs");
    }

    #[test]
    fn absolute_ok() {
        let vfs = super::new("tests").unwrap();
        assert_eq!(&vfs.absolute("test1"), Path::new("/home/dvshapkin/projects/rust/toolbox/tests/test1"));
    }

    #[test]
    fn other() {
        let vfs = super::new(".").unwrap();
        for component in vfs.root.components() {
            println!("{:?}", component);
        }
    }
}
