//! ` `

// @<Put each...@>=
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1141($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("eqno",eq_no,0);
    primitive(globals, crate::strpool_str!("eqno"), eq_no, 0);
    // @!@:eq_no_}{\.{\\eqno} primitive@>
    // primitive("leqno",eq_no,1);
    primitive(globals, crate::strpool_str!("leqno"), eq_no, 1);
    // @!@:leq_no_}{\.{\\leqno} primitive@>

    use crate::section_0208::*;
    use crate::section_0264::primitive;
}}
