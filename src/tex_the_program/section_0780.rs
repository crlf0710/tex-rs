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
pub(crate) const end_template_token: cur_tok_repr =
    cur_tok_type::from_cs(frozen_end_template as _).get();

// @<Put each of \TeX's primitives into the hash table@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_0780($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("span",tab_mark,span_code);@/
    primitive(
        globals,
        crate::strpool_str!("span"),
        tab_mark,
        span_code as _,
    );
    // @!@:span_}{\.{\\span} primitive@>
    // primitive("cr",car_ret,cr_code);
    primitive(globals, crate::strpool_str!("cr"), car_ret, cr_code as _);
    // @!@:cr_}{\.{\\cr} primitive@>
    // text(frozen_cr):="cr"; eqtb[frozen_cr]:=eqtb[cur_val];@/
    text!(globals, frozen_cr as pointer) = crate::strpool_str!("cr").get() as _;
    globals.eqtb[frozen_cr as pointer] = globals.eqtb[globals.cur_val as pointer];
    // primitive("crcr",car_ret,cr_cr_code);
    primitive(
        globals,
        crate::strpool_str!("crcr"),
        car_ret,
        cr_cr_code as _,
    );
    // @!@:cr_cr_}{\.{\\crcr} primitive@>
    // text(frozen_end_template):="endtemplate"; text(frozen_endv):="endtemplate";
    // @.endtemplate@>
    text!(globals, frozen_end_template as pointer) = crate::strpool_str!("endtemplate").get() as _;
    text!(globals, frozen_endv as pointer) = crate::strpool_str!("endtemplate").get() as _;
    // eq_type(frozen_endv):=endv; equiv(frozen_endv):=null_list;
    eq_type!(globals, frozen_endv as pointer) = endv as _;
    equiv!(globals, frozen_endv as pointer) = null_list;
    // eq_level(frozen_endv):=level_one;@/
    eq_level!(globals, frozen_endv as pointer) = level_one;
    // eqtb[frozen_end_template]:=eqtb[frozen_endv];
    globals.eqtb[frozen_end_template as pointer] = globals.eqtb[frozen_endv as pointer];
    // eq_type(frozen_end_template):=end_template;
    eq_type!(globals, frozen_end_template as pointer) = end_template as _;
}}

use crate::section_0004::TeXGlobals;
use crate::section_0113::halfword;
use crate::section_0115::pointer;
use crate::section_0162::null_list;
use crate::section_0207::*;
use crate::section_0210::end_template;
use crate::section_0221::eq_level;
use crate::section_0221::eq_type;
use crate::section_0221::equiv;
use crate::section_0221::level_one;
use crate::section_0222::frozen_cr;
use crate::section_0222::frozen_end_template;
use crate::section_0222::frozen_endv;
use crate::section_0256::text;
use crate::section_0264::primitive;
use crate::section_0297::cur_tok_repr;
use crate::section_0297::cur_tok_type;
