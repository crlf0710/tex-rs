//! @ The primitives \.{\\number}, \.{\\romannumeral}, \.{\\string}, \.{\\meaning},
//! \.{\\fontname}, and \.{\\jobname} are defined as follows.
//
// @d number_code=0 {command code for \.{\\number}}
// @d roman_numeral_code=1 {command code for \.{\\romannumeral}}
// @d string_code=2 {command code for \.{\\string}}
// @d meaning_code=3 {command code for \.{\\meaning}}
// @d font_name_code=4 {command code for \.{\\fontname}}
// @d job_name_code=5 {command code for \.{\\jobname}}

#[derive(Copy, Clone)]
pub(crate) enum convert_code_kind {
    /// command code for `\number`
    number_code = 0,
    /// command code for `\romannumeral`
    roman_numeral_code = 1,
    /// command code for `\string`
    string_code = 2,
    /// command code for `\meaning`
    meaning_code = 3,
    /// command code for `\fontname`
    font_name_code = 4,
    /// command code for `\jobname`
    job_name_code = 5,
}

impl From<u8> for convert_code_kind {
    fn from(val: u8) -> convert_code_kind {
        match val {
            0 => convert_code_kind::number_code,
            1 => convert_code_kind::roman_numeral_code,
            2 => convert_code_kind::string_code,
            3 => convert_code_kind::meaning_code,
            4 => convert_code_kind::font_name_code,
            5 => convert_code_kind::job_name_code,
            _ => unreachable!()
        }
    }
}

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0468(globals: &mut TeXGlobals) {
    // primitive("number",convert,number_code);@/
    primitive(globals, strpool_str!("number"), convert, convert_code_kind::number_code as _);
    // @!@:number_}{\.{\\number} primitive@>
    // primitive("romannumeral",convert,roman_numeral_code);@/
    // @!@:roman_numeral_}{\.{\\romannumeral} primitive@>
    // primitive("string",convert,string_code);@/
    primitive(globals, strpool_str!("string"), convert, convert_code_kind::string_code as _);
    // @!@:string_}{\.{\\string} primitive@>
    // primitive("meaning",convert,meaning_code);@/
    // @!@:meaning_}{\.{\\meaning} primitive@>
    // primitive("fontname",convert,font_name_code);@/
    // @!@:font_name_}{\.{\\fontname} primitive@>
    // primitive("jobname",convert,job_name_code);@/
    // @!@:job_name_}{\.{\\jobname} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use crate::section_0210::convert;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
