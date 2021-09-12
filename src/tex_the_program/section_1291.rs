//! ` `

// @d show_code=0 { \.{\\show} }
// @d show_box_code=1 { \.{\\showbox} }
// @d show_the_code=2 { \.{\\showthe} }
// @d show_lists_code=3 { \.{\\showlists} }
pub(crate) enum show_kind {
    /// `\show`
    show_code = 0,
    /// `\showbox`
    show_box_code = 1,
    /// `\showthe`
    show_the_code = 2,
    /// `\showlists`
    show_lists_code = 3,
}

// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1291($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("show",xray,show_code);
    primitive(
        globals,
        crate::strpool_str!("show"),
        xray,
        show_kind::show_code as _,
    );
    // @!@:show_}{\.{\\show} primitive@>
    // primitive("showbox",xray,show_box_code);
    primitive(
        globals,
        crate::strpool_str!("showbox"),
        xray,
        show_kind::show_box_code as _,
    );
    // @!@:show_box_}{\.{\\showbox} primitive@>
    // primitive("showthe",xray,show_the_code);
    primitive(
        globals,
        crate::strpool_str!("showthe"),
        xray,
        show_kind::show_the_code as _,
    );
    // @!@:show_the_}{\.{\\showthe} primitive@>
    // primitive("showlists",xray,show_lists_code);
    primitive(
        globals,
        crate::strpool_str!("showlists"),
        xray,
        show_kind::show_lists_code as _,
    );
    // @!@:show_lists_code_}{\.{\\showlists} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0208::xray;
use crate::section_0264::primitive;
