//! ` `

// @ @<Last-minute...@>=

// FIXME: Using short name to workaround https://github.com/dtolnay/linkme/issues/35
#[distributed_slice]
pub(crate) static PRIM2HT: [fn(&mut TeXGlobals)] = [..];

macro_rules! Put_each_of_TeX_s_primitivies_into_the_hash_table {
    ($globals:expr) => {
        for f in PRIM2HT {
            f($globals);
        }
    };
}

// @!init procedure init_prim; {initialize all the primitives}
/// initialize all the primitives
#[cfg(feature = "initex")]
pub(crate) fn init_prim(globals: &mut TeXGlobals) {
    // begin no_new_control_sequence:=false;
    globals.no_new_control_sequence = false;
    // @<Put each...@>;
    Put_each_of_TeX_s_primitivies_into_the_hash_table!(globals);
    // no_new_control_sequence:=true;
    globals.no_new_control_sequence = true;
    // end;
    // tini
}


use crate::section_0004::TeXGlobals;
use linkme::distributed_slice;
