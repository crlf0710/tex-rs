//! @ We enter `\.{\\span}' into |eqtb| with |tab_mark| as its command code,
//! and with |span_code| as the command modifier. This makes \TeX\ interpret it
//! essentially the same as an alignment delimiter like `\.\&', yet it is
//! recognizably different when we need to distinguish it from a normal delimiter.
//! It also turns out to be useful to give a special |cr_code| to `\.{\\cr}',
//! and an even larger |cr_cr_code| to `\.{\\crcr}'.
//!
//! The end of a template is represented by two ``frozen'' control sequences
//! called \.{\\endtemplate}. The first has the command code |end_template|, which
//! is |>outer_call|, so it will not easily disappear in the presence of errors.
//! The |get_x_token| routine converts the first into the second, which has |endv|
//! as its command code.
//
// @d span_code=256 {distinct from any character}
/// distinct from any character
pub(crate) const span_code: halfword = 256;
// @d cr_code=257 {distinct from |span_code| and from any character}
/// distinct from `span_code` and from any character
pub(crate) const cr_code: halfword = 257;
// @d cr_cr_code=cr_code+1 {this distinguishes \.{\\crcr} from \.{\\cr}}
/// this distinguishes `\crcr` from `\cr`
pub(crate) const cr_cr_code: halfword = cr_code + 1;
// @d end_template_token==cs_token_flag+frozen_end_template
//
// @<Put each of \TeX's primitives into the hash table@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0780(globals: &mut TeXGlobals) {
    // primitive("span",tab_mark,span_code);@/
    // @!@:span_}{\.{\\span} primitive@>
    // primitive("cr",car_ret,cr_code);
    primitive(globals, strpool_str!("cr"), car_ret, cr_code as _);
    // @!@:cr_}{\.{\\cr} primitive@>
    // text(frozen_cr):="cr"; eqtb[frozen_cr]:=eqtb[cur_val];@/
    text!(globals, frozen_cr as pointer) = strpool_str!("cr").get() as _;
    globals.eqtb[frozen_cr as pointer] = globals.eqtb[globals.cur_val as pointer];
    // primitive("crcr",car_ret,cr_cr_code);
    primitive(globals, strpool_str!("crcr"), car_ret, cr_cr_code as _);
    // @!@:cr_cr_}{\.{\\crcr} primitive@>
    // text(frozen_end_template):="endtemplate"; text(frozen_endv):="endtemplate";
    // eq_type(frozen_endv):=endv; equiv(frozen_endv):=null_list;
    // eq_level(frozen_endv):=level_one;@/
    // eqtb[frozen_end_template]:=eqtb[frozen_endv];
    // eq_type(frozen_end_template):=end_template;
}

use crate::section_0004::TeXGlobals;
use crate::section_0113::halfword;
use crate::section_0115::pointer;
use crate::section_0207::*;
use crate::section_0222::frozen_cr;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
