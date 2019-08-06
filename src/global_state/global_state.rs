use std::fmt;
use std::str;

use fnv::FnvHashMap;
use sdbm::sdbm_hash;

use super::name::*;

pub struct GlobalState {
    // TODO(jez) Convert to Vec<String>, chunk into pages
    pub(super) strings: String,
    pub(super) names: Vec<Name>,
    pub(super) hashes_to_name_ids: FnvHashMap<u32, usize>
}

impl fmt::Debug for GlobalState {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "┌─ GlobalState ──\n")?;
        write!(out, "│ strings:            {}\n",   self.strings)?;
        write!(out, "│ names:              {:?}\n", self.names)?;
        write!(out, "│ hashes_to_name_ids: {:?}\n", self.hashes_to_name_ids)?;
        write!(out, "└────────────────")
    }
}

impl GlobalState {
    pub fn enter_name(&mut self, string: &str) -> NameRef {
        let hash = sdbm_hash(string);

        if let Some(idx) = self.hashes_to_name_ids.get(&hash) {
            // TODO(jez) Debug check: check for collisions?
            return NameRef { idx: *idx };
        }

        let offset = self.strings.len();
        self.strings.push_str(string);

        let idx = self.names.len();
        self.names.push(Name { offset, len: string.len(), idx });
        self.hashes_to_name_ids.insert(hash, idx);

        NameRef { idx }
    }

    pub fn new() -> GlobalState {
        let strings = String::new();
        let mut names = Vec::new();
        let hashes_to_name_ids = FnvHashMap::default();
        let no_name = Name { offset: 0, len: 0, idx: 0 };
        names.push(no_name);
        GlobalState { strings, names, hashes_to_name_ids }
    }
}
