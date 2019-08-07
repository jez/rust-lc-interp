use std::path::{PathBuf, Path};

use super::global_state::*;

// TODO(jez) Sorbet fits a FileRef into a single u16

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct FileRef {
    pub(super) idx: usize,
}

impl FileRef {
    pub fn exists(&self) -> bool {
        self.idx == 0
    }

    pub fn data<'a>(&self, gs: &'a GlobalState) -> &'a File {
        debug_assert!(self.exists(), "FileRef::data for non-existent FileRef");
        &gs.files[self.idx]
    }
}

#[derive(Debug)]
pub struct File {
    pub(super) path_buf: PathBuf,
    pub(super) contents: String,
    pub(super) idx: usize,
}

impl File {
    pub fn to_file_ref(&self) -> FileRef {
        FileRef { idx: self.idx }
    }

    // Forces immutable access to a File's data
    pub fn path(&self) -> &Path {
        self.path_buf.as_path()
    }
    pub fn contents(&self) -> &str {
        self.contents.as_str()
    }
}
