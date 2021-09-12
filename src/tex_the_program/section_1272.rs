//! @ Files for \.{\\read} are opened and closed by the |in_stream| command.
//
// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1272($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("openin",in_stream,1);
    primitive(globals, crate::strpool_str!("openin"), in_stream, 1);
    // @!@:open_in_}{\.{\\openin} primitive@>
    // primitive("closein",in_stream,0);
    primitive(globals, crate::strpool_str!("closein"), in_stream, 0);
    // @!@:close_in_}{\.{\\closein} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0208::in_stream;
use crate::section_0264::primitive;
