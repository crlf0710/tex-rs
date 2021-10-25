//! @ Extensions might introduce new command codes; but it's best to use
//! |extension| with a modifier, whenever possible, so that |main_control|
//! stays the same.
//
// @d immediate_code=4 {command modifier for \.{\\immediate}}
/// command modifier for `\immediate`
pub(crate) const immediate_code: quarterword = 4;
// @d set_language_code=5 {command modifier for \.{\\setlanguage}}
/// command modifier for `\setlanguage`
pub(crate) const set_language_code: quarterword = 5;

// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1344($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("openout",extension,open_node);@/
    primitive(
        globals,
        crate::strpool_str!("openout"),
        extension,
        open_node as _,
    );
    // @!@:open_out_}{\.{\\openout} primitive@>
    // primitive("write",extension,write_node); write_loc:=cur_val;@/
    primitive(
        globals,
        crate::strpool_str!("write"),
        extension,
        write_node as _,
    );
    globals.write_loc = globals.cur_val as pointer;
    // @!@:write_}{\.{\\write} primitive@>
    // primitive("closeout",extension,close_node);@/
    primitive(
        globals,
        crate::strpool_str!("closeout"),
        extension,
        close_node as _,
    );
    // @!@:close_out_}{\.{\\closeout} primitive@>
    // primitive("special",extension,special_node);@/
    primitive(
        globals,
        crate::strpool_str!("special"),
        extension,
        special_node as _,
    );
    // @!@:special_}{\.{\\special} primitive@>
    // primitive("immediate",extension,immediate_code);@/
    primitive(
        globals,
        crate::strpool_str!("immediate"),
        extension,
        immediate_code as _,
    );
    // @!@:immediate_}{\.{\\immediate} primitive@>
    // primitive("setlanguage",extension,set_language_code);@/
    primitive(
        globals,
        crate::strpool_str!("setlanguage"),
        extension,
        set_language_code as _,
    );
    // @!@:set_language_}{\.{\\setlanguage} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use crate::section_0208::extension;
use crate::section_0264::primitive;
use crate::section_1341::close_node;
use crate::section_1341::open_node;
use crate::section_1341::special_node;
use crate::section_1341::write_node;
