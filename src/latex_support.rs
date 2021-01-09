const unimplemented_code: quarterword = if_case_code + 1; 

#[distributed_slice(PRIM2HT)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_latex_support(
    globals: &mut TeXGlobals,
) {
    primitive(globals, strpool_str!("ifcsname"), if_test, unimplemented_code as _);
}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use crate::section_0210::if_test;
use crate::section_0224::*;
use crate::section_0264::primitive;
use crate::section_0487::if_case_code;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
