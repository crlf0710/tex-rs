//! @ The processing of \.{\\input} involves the |start_input| subroutine,
//! which will be declared later; the processing of \.{\\endinput} is trivial.
//
// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_0376($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("input",input,0);@/
    primitive(globals, crate::strpool_str!("input"), input, 0);
    // @!@:input_}{\.{\\input} primitive@>
    // primitive("endinput",input,1);@/
    primitive(globals, crate::strpool_str!("endinput"), input, 1);
    // @!@:end_input_}{\.{\\endinput} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0210::input;
use crate::section_0264::primitive;
