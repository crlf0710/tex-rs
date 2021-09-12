//! @ Both \.{\\let} and \.{\\futurelet} share the command code |let|.
//
// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1219($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("let",let,normal);@/
    primitive(
        globals,
        crate::strpool_str!("let"),
        r#let,
        let_kind::normal as _,
    );
    // @!@:let_}{\.{\\let} primitive@>
    // primitive("futurelet",let,normal+1);@/
    primitive(
        globals,
        crate::strpool_str!("futurelet"),
        r#let,
        let_kind::futurelet as _,
    );
    // @!@:future_let_}{\.{\\futurelet} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0135::let_kind;
use crate::section_0209::*;
use crate::section_0264::primitive;
