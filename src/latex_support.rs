const unimplemented_code: quarterword = if_case_code + 1;

pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_latex_support($globals:expr) {{
    let globals = &mut *$globals;
    primitive(
        globals,
        crate::strpool_str!("ifcsname"),
        if_test,
        unimplemented_code as _,
    );

    globals.error_line = 79;
    globals.half_error_line = 50;
    globals.max_print_line = 79;
}}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use crate::section_0210::if_test;
use crate::section_0224::*;
use crate::section_0264::primitive;
use crate::section_0487::if_case_code;
