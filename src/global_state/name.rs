use super::global_state::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct NameRef {
    pub(super) idx: usize,
}

impl NameRef {
    pub fn exists(&self) -> bool {
        self.idx != 0
    }

    pub fn data<'a>(&self, gs: &'a GlobalState) -> &'a Name {
        debug_assert!(self.exists(), "NameRef::data for non-existent NameRef");
        &gs.names[self.idx]
    }
}

#[derive(Debug)]
pub struct Name {
    pub(super) offset: usize,
    pub(super) len: usize,
    pub(super) idx: usize,
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

