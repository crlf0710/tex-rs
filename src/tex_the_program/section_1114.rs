//! @ Discretionary nodes are easy in the common case `\.{\\-}', but in the
//! general case we must process three braces full of items.
//
// @<Put each...@>=
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1114($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("-",discretionary,1);
    primitive(globals, crate::strpool_str!("-"), discretionary, 1);
    // @!@:Single-character primitives -}{\quad\.{\\-}@>
    // primitive("discretionary",discretionary,0);
    primitive(
        globals,
        crate::strpool_str!("discretionary"),
        discretionary,
        0,
    );
    // @!@:discretionary_}{\.{\\discretionary} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0208::discretionary;
use crate::section_0264::primitive;
