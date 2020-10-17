//! ` `

// @ @<Last-minute...@>=

#[distributed_slice]
pub(crate) static PUT_EACH_OF_TEX_S_PRIMITIVIES_INTO_THE_HASH_TABLE: [fn(&mut TeXGlobals)] = [..];

macro_rules! Put_each_of_TeX_s_primitivies_into_the_hash_table {
    ($globals:expr) => {
        for f in PUT_EACH_OF_TEX_S_PRIMITIVIES_INTO_THE_HASH_TABLE {
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
