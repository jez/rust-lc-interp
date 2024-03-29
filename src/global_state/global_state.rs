use std::fmt;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};
use std::str;

use fnv::FnvHashMap;
use fnv::FnvHasher;

use super::file::*;
use super::name::*;

pub struct GlobalState {
    // TODO(jez) Convert to Vec<String>, chunk into pages
    pub(super) strings: String,
    pub(super) names: Vec<Name>,
    pub(super) hashes_to_name_ids: FnvHashMap<u64, usize>,

    pub(super) files: Vec<File>,
    pub(super) hashes_to_file_ids: FnvHashMap<u64, usize>,
}

impl fmt::Debug for GlobalState {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "┌─ GlobalState ──\n")?;
        write!(out, "│ strings:            {}\n", self.strings)?;
        write!(out, "│ names:              {:?}\n", self.names)?;
        write!(
            out,
            "│ hashes_to_name_ids: {:?}\n",
            self.hashes_to_name_ids
        )?;
        write!(out, "│ files:              {:?}\n", self.files)?;
        write!(
            out,
            "│ hashes_to_file_ids: {:?}\n",
            self.hashes_to_file_ids
        )?;
        write!(out, "└────────────────")
    }
}

impl GlobalState {
    pub fn enter_name(&mut self, string: &str) -> NameRef {
        let mut hasher = FnvHasher::default();
        string.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(idx) = self.hashes_to_name_ids.get(&hash) {
            // TODO(jez) Debug check: check for collisions?
            return NameRef { idx: *idx };
        }

        let offset = self.strings.len();
        self.strings.push_str(string);

        let idx = self.names.len();
        self.names.push(Name {
            offset,
            len: string.len(),
            idx,
        });
        self.hashes_to_name_ids.insert(hash, idx);

        NameRef { idx }
    }

    pub fn enter_file(&mut self, path: &Path) -> io::Result<FileRef> {
        let path_buf = fs::canonicalize(path)?;

        let mut hasher = FnvHasher::default();
        path_buf.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(idx) = self.hashes_to_file_ids.get(&hash) {
            // TODO(jez) Debug check: check for collisions?
            return Ok(FileRef { idx: *idx });
        }

        // TODO(jez) Read files only right before parsing them
        let contents = fs::read_to_string(&path_buf)?;

        let idx = self.files.len();
        self.files.push(File {
            path_buf,
            contents,
            idx,
        });

        Ok(FileRef { idx })
    }

    pub fn new() -> GlobalState {
        let strings = String::new();
        let mut names = Vec::new();
        let hashes_to_name_ids = FnvHashMap::default();

        let no_name = Name {
            offset: 0,
            len: 0,
            idx: 0,
        };
        names.push(no_name);

        let mut files = Vec::new();
        let hashes_to_file_ids = FnvHashMap::default();

        let no_file = File {
            path_buf: PathBuf::new(),
            contents: String::new(),
            idx: 0,
        };
        files.push(no_file);

        GlobalState {
            strings,
            names,
            hashes_to_name_ids,
            files,
            hashes_to_file_ids,
        }
    }
}
