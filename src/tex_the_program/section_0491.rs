//! ` `

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0491(globals: &mut TeXGlobals) {
    // primitive("fi",fi_or_else,fi_code);
    primitive(globals, crate::strpool_str!("fi"), fi_or_else, fi_code as _);
    // @!@:fi_}{\.{\\fi} primitive@>
    // text(frozen_fi):="fi"; eqtb[frozen_fi]:=eqtb[cur_val];
    text!(globals, frozen_fi as pointer) = crate::strpool_str!("fi").get() as _;
    globals.eqtb[frozen_fi as pointer] = globals.eqtb[globals.cur_val as pointer];
    // primitive("or",fi_or_else,or_code);
    primitive(globals, crate::strpool_str!("or"), fi_or_else, or_code as _);
    // @!@:or_}{\.{\\or} primitive@>
    // primitive("else",fi_or_else,else_code);
    primitive(
        globals,
        crate::strpool_str!("else"),
        fi_or_else,
        else_code as _,
    );
    // @!@:else_}{\.{\\else} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0210::*;
use crate::section_0222::frozen_fi;
use crate::section_0256::text;
use crate::section_0264::primitive;
use crate::section_0489::*;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
