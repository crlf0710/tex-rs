//! ` `
//!
//! An operation like `\.{\\over}' causes the current mlist to go into a
//! state of suspended animation: |incompleat_noad| points to a |fraction_noad|
//! that contains the mlist-so-far as its numerator, while the denominator
//! is yet to come. Finally when the mlist is finished, the denominator will
//! go into the incompleat fraction noad, and that noad will become the
//! whole formula, unless it is surrounded by `\.{\\left}' and `\.{\\right}'
//! delimiters.
//
// @d above_code=0 { `\.{\\above}' }
/// `\above`
pub(crate) const above_code: quarterword = 0;
// @d over_code=1 { `\.{\\over}' }
/// `\over`
pub(crate) const over_code: quarterword = 1;
// @d atop_code=2 { `\.{\\atop}' }
/// `\atop`
pub(crate) const atop_code: quarterword = 2;
// @d delimited_code=3 { `\.{\\abovewithdelims}', etc.}
/// `\abovewithdelims`, etc.
pub(crate) const delimited_code: quarterword = 3;

// @<Put each...@>=
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1178($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("above",above,above_code);@/
    primitive(
        globals,
        crate::strpool_str!("above"),
        above,
        above_code as _,
    );
    // @!@:above_}{\.{\\above} primitive@>
    // primitive("over",above,over_code);@/
    primitive(globals, crate::strpool_str!("over"), above, over_code as _);
    // @!@:over_}{\.{\\over} primitive@>
    // primitive("atop",above,atop_code);@/
    primitive(globals, crate::strpool_str!("atop"), above, atop_code as _);
    // @!@:atop_}{\.{\\atop} primitive@>
    // primitive("abovewithdelims",above,delimited_code+above_code);@/
    primitive(
        globals,
        crate::strpool_str!("abovewithdelims"),
        above,
        (delimited_code + above_code) as _,
    );
    // @!@:above_with_delims_}{\.{\\abovewithdelims} primitive@>
    // primitive("overwithdelims",above,delimited_code+over_code);@/
    primitive(
        globals,
        crate::strpool_str!("overwithdelims"),
        above,
        (delimited_code + over_code) as _,
    );
    // @!@:over_with_delims_}{\.{\\overwithdelims} primitive@>
    // primitive("atopwithdelims",above,delimited_code+atop_code);
    primitive(
        globals,
        crate::strpool_str!("atopwithdelims"),
        above,
        (delimited_code + atop_code) as _,
    );
    // @!@:atop_with_delims_}{\.{\\atopwithdelims} primitive@>

    use crate::section_0208::*;
    use crate::section_0264::primitive;
}}

use crate::section_0113::quarterword;
