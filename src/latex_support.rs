const unimplemented_code: quarterword = if_case_code + 1;

#[globals_struct_field(TeXGlobals)]
pub(crate) static latex_support_enabled: bool = false;

#[cfg(feature = "initex")]
pub(crate) fn external_primitive(
    globals: &mut TeXGlobals,
    s: &'static str,
    c: quarterword,
    o: halfword,
) {
    assert!(s.chars().all(|ch| ch.is_ascii_graphic()));
    let l = s.chars().count();
    assert!(l > 0);
    if l == 1 {
        globals.cur_val = (s.chars().nth(0).unwrap() as word + single_base) as _;
    } else {
        for (idx, ch) in s.chars().enumerate() {
            globals.buffer[idx as u16] = ASCII_code::from((ch as u32) as integer);
        }
        globals.cur_val = id_lookup(globals, 0, l as _) as _;
    }
    eq_level!(globals, globals.cur_val as u32) = level_one;
    eq_type!(globals, globals.cur_val as u32) = c;
    equiv!(globals, globals.cur_val as u32) = o;
    // end;
    // tini
}

pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_latex_support($globals:expr) {{
    let globals = &mut *$globals;
    if globals.latex_support_enabled {
        external_primitive(globals, "ifcsname", if_test, unimplemented_code as _);
    }
    // globals.error_line = 79;
    // globals.half_error_line = 50;
    // globals.max_print_line = 79;
}}

use crate::pascal::integer;
use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0113::halfword;
use crate::section_0113::quarterword;
use crate::section_0210::if_test;
use crate::section_0221::eq_level;
use crate::section_0221::eq_type;
use crate::section_0221::equiv;
use crate::section_0221::level_one;
use crate::section_0222::single_base;
use crate::section_0224::*;
use crate::section_0259::id_lookup;
use crate::section_0264::primitive;
use crate::section_0487::if_case_code;

use globals_struct::globals_struct_field;
