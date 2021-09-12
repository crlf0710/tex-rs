//! @ New hyphenation data is loaded by the |hyph_data| command.
//
// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1250($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("hyphenation",hyph_data,0);
    primitive(globals, crate::strpool_str!("hyphenation"), hyph_data, 0);
    // @!@:hyphenation_}{\.{\\hyphenation} primitive@>
    // primitive("patterns",hyph_data,1);
    primitive(globals, crate::strpool_str!("patterns"), hyph_data, 1);
    // @!@:patterns_}{\.{\\patterns} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0209::hyph_data;
use crate::section_0264::primitive;
