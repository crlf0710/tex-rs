//! ` `
// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1277($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("message",message,0);
    primitive(globals, crate::strpool_str!("message"), message, 0);
    // @!@:message_}{\.{\\message} primitive@>
    // primitive("errmessage",message,1);
    primitive(globals, crate::strpool_str!("errmessage"), message, 1);
    // @!@:err_message_}{\.{\\errmessage} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0208::*;
use crate::section_0264::primitive;
