//! @ Either \.{\\dump} or \.{\\end} will cause |main_control| to enter the
//! endgame, since both of them have `|stop|' as their command code.
//!
//! @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1052($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("end",stop,0);@/
    // @!@:end_}{\.{\\end} primitive@>
    primitive(globals, crate::strpool_str!("end"), stop, 0);
    // primitive("dump",stop,1);@/
    // @!@:dump_}{\.{\\dump} primitive@>
    primitive(globals, crate::strpool_str!("dump"), stop, 1);
}}

use crate::section_0004::TeXGlobals;
use crate::section_0207::*;
use crate::section_0264::primitive;
