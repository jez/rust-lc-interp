use std::str;

use fnv::FnvHashMap;
use sdbm::sdbm_hash;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct NameRef {
    idx: usize,
}

impl NameRef {
    pub fn exists(&self) -> bool {
        self.idx == 0
    }

    pub fn data<'a>(&self, gs: &'a GlobalState) -> &'a Name {
        debug_assert!(self.exists(), "NameRef::data for non-existent NameRef");
        &gs.names[self.idx]
    }
}

#[derive(Debug)]
pub struct Name {
    offset: usize,
    len: usize,
    idx: usize,
}

impl Name {
    pub fn to_name_ref(&self) -> NameRef {
        NameRef { idx: self.idx }
    }

    pub fn show<'a>(&self, gs: &'a GlobalState) -> &'a str {
        debug_assert!(self.to_name_ref().exists(), "Name::show for non-existent Name");
        &gs.strings[self.offset .. (self.offset + self.len)]
    }
}

pub struct GlobalState {
    // TODO(jez) Convert to Vec<String>, chunk into pages
    strings: String,
    names: Vec<Name>,
    hashes_to_name_ids: FnvHashMap<u32, usize>
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
