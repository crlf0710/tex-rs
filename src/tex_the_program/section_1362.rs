//! ` `
// @d adv_past(#)==@+if subtype(#)=language_node then
macro_rules! adv_past {
    ($globals:expr, $v:expr) => {{
        if subtype!($globals, $v) == language_node {
            // begin cur_lang:=what_lang(#); l_hyf:=what_lhm(#); r_hyf:=what_rhm(#);@+end
            $globals.cur_lang = ASCII_code::from(what_lang!($globals, $v) as integer);
            $globals.l_hyf = what_lhm!($globals, $v) as _;
            $globals.r_hyf = what_rhm!($globals, $v) as _;
        }
        use crate::section_0018::ASCII_code;
        use crate::section_1341::language_node;
    }};
}
