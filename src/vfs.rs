//! Virtual file system allows you to work with relative file paths in a convenient way

use std::path::{Path, PathBuf};
use std::io;

/// Create new `VirtualFileSystem`
///
/// `root` - base directory for VFS.
/// A `root` path must exists; if not, return value will be `None`.
/// If it's a relative path, then it will be normalized.
pub fn new(root: &str) -> Option<VirtualFileSystem> {
    match Path::new(root).canonicalize() {
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
    pub fn chroot(&mut self, new_root: &str) -> io::Result<()> {
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
    pub fn absolute(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        }
    }

    /// Convert absolute `path` to relative
    ///
    /// If `path` is not absolute, then return `None`.
    /// If `path` is equal to `root`, then return `.` (current).
    /// If `root = "/foo/bar"` and `path = "/foo/bar/more"`, then return `more`.
    pub fn relative(&self, path: &Path) -> Option<PathBuf> {
        if path.is_absolute() {
            if path == self.root {
                return Some(PathBuf::from("."));
            } else {
                if path.starts_with(&self.root) {
                    return Some(path.strip_prefix(&self.root).unwrap().to_path_buf());
                }
            }
        }
        None
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
        assert_eq!(vfs.root, Path::new("/home/dvshapkin/projects/rust/vfs").canonicalize().unwrap());

        assert_eq!(vfs.relative(Path::new("/home/dvshapkin/projects/rust/vfs")).unwrap(), Path::new("."));
        assert_eq!(vfs.relative(Path::new("/home/dvshapkin/projects/rust/vfs/Cargo.toml")).unwrap(), Path::new("Cargo.toml"));
    }
}
