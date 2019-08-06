//
// This whole module is set up so that submodules have access to private things in peer modules,
// but that outside of this module, those things are private.
//

//
// This is to make it look like global_state is actually flat, and that these visibility tricks are
// not actually happening.
//
mod global_state;
pub use global_state::*;

mod name;
pub use name::*;
