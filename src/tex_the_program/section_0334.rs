//! ` `

// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_0334($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("par",par_end,256); {cf.\ |scan_file_name|}
    /// cf. `scan_file_name`
    primitive(globals, crate::strpool_str!("par"), par_end, 256);
    // @!@:par_}{\.{\\par} primitive@>
    // par_loc:=cur_val; par_token:=cs_token_flag+par_loc;
    globals.par_loc = globals.cur_val as _;
    globals.par_token = cur_tok_type::from_cs(globals.par_loc);
}}

use crate::section_0004::TeXGlobals;
use crate::section_0207::par_end;
use crate::section_0208::*;
use crate::section_0264::primitive;
use crate::section_0297::cur_tok_type;
