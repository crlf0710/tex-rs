//! ` `

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0334(globals: &mut TeXGlobals) {
    // primitive("par",par_end,256); {cf.\ |scan_file_name|}
    /// cf. `scan_file_name`
    primitive(globals, strpool_str!("par"), par_end, 256);
    // @!@:par_}{\.{\\par} primitive@>
    // par_loc:=cur_val; par_token:=cs_token_flag+par_loc;
    globals.par_loc = globals.cur_val as _;
    globals.par_token = cur_tok_type::from_cs(globals.par_loc);
}

use crate::section_0004::TeXGlobals;
use crate::section_0208::*;
use crate::section_0264::primitive;
use crate::section_0297::cur_tok_type;
use crate::section_0207::par_end;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
