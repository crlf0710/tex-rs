//! ` `

// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1254($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("hyphenchar",assign_font_int,0);
    primitive(
        globals,
        crate::strpool_str!("hyphenchar"),
        assign_font_int,
        0,
    );
    // @!@:hyphen_char_}{\.{\\hyphenchar} primitive@>
    // primitive("skewchar",assign_font_int,1);
    primitive(globals, crate::strpool_str!("skewchar"), assign_font_int, 1);
    // @!@:skew_char_}{\.{\\skewchar} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0209::*;
use crate::section_0264::primitive;
