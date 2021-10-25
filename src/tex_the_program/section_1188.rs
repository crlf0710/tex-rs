//! ` `
//! We have dealt with all constructions of math mode except `\.{\\left}' and
//! `\.{\\right}', so the picture is completed by the following sections of
//! the program.
//
// @<Put each...@>=
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1188($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("left",left_right,left_noad);
    primitive(
        globals,
        crate::strpool_str!("left"),
        left_right,
        left_noad as _,
    );
    // @!@:left_}{\.{\\left} primitive@>
    // primitive("right",left_right,right_noad);
    primitive(
        globals,
        crate::strpool_str!("right"),
        left_right,
        right_noad as _,
    );
    // @!@:right_}{\.{\\right} primitive@>
    // text(frozen_right):="right"; eqtb[frozen_right]:=eqtb[cur_val];
    text!(globals, frozen_right as u16) = crate::strpool_str!("right").get() as _;
    globals.eqtb[frozen_right as u16] = globals.eqtb[globals.cur_val as u16];

    use crate::section_0208::*;
    use crate::section_0222::frozen_right;
    use crate::section_0256::text;
    use crate::section_0264::primitive;
    use crate::section_0687::*;
}}
