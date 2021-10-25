//! ` `
//! The routine that inserts a |style_node| holds no surprises.
//
// @<Put each...@>=
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1169($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("displaystyle",math_style,display_style);
    primitive(
        globals,
        crate::strpool_str!("displaystyle"),
        math_style,
        style_node_subtype::display_style as _,
    );
    // @!@:display_style_}{\.{\\displaystyle} primitive@>
    // primitive("textstyle",math_style,text_style);
    primitive(
        globals,
        crate::strpool_str!("textstyle"),
        math_style,
        style_node_subtype::text_style as _,
    );
    // @!@:text_style_}{\.{\\textstyle} primitive@>
    // primitive("scriptstyle",math_style,script_style);
    primitive(
        globals,
        crate::strpool_str!("scriptstyle"),
        math_style,
        style_node_subtype::script_style as _,
    );
    // @!@:script_style_}{\.{\\scriptstyle} primitive@>
    // primitive("scriptscriptstyle",math_style,script_script_style);
    primitive(
        globals,
        crate::strpool_str!("scriptscriptstyle"),
        math_style,
        style_node_subtype::script_script_style as _,
    );
    // @!@:script_script_style_}{\.{\\scriptscriptstyle} primitive@>
    use crate::section_0208::*;
    use crate::section_0264::primitive;
    use crate::section_0688::style_node_subtype;
}}
