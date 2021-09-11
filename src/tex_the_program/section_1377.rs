//! ` `

// @<Implement \.{\\setlanguage}@>=
pub(crate) macro Implement_setlanguage($globals:expr) {{
    // if abs(mode)<>hmode then report_illegal_case
    if mode!($globals).get().abs() != hmode {
        report_illegal_case($globals)?;
    }
    // else begin new_whatsit(language_node,small_node_size);
    else {
        new_whatsit($globals, language_node.into(), small_node_size.into())?;
        // scan_int;
        scan_int($globals)?;
        // if cur_val<=0 then clang:=0
        if $globals.cur_val <= 0 {
            clang!($globals) = 0;
        }
        // else if cur_val>255 then clang:=0
        else if $globals.cur_val > 255 {
            clang!($globals) = 0;
        }
        // else clang:=cur_val;
        else {
            clang!($globals) = $globals.cur_val as _;
        }
        // what_lang(tail):=clang;
        what_lang!($globals, tail!($globals)) = clang!($globals);
        // what_lhm(tail):=norm_min(left_hyphen_min);
        what_lhm!($globals, tail!($globals)) = norm_min(left_hyphen_min!($globals)).get();
        // what_rhm(tail):=norm_min(right_hyphen_min);
        what_rhm!($globals, tail!($globals)) = norm_min(right_hyphen_min!($globals)).get();
        // end
    }
    use crate::section_0141::small_node_size;
    use crate::section_0211::hmode;
    use crate::section_0213::clang;
    use crate::section_0213::mode;
    use crate::section_0213::tail;
    use crate::section_0236::left_hyphen_min;
    use crate::section_0236::right_hyphen_min;
    use crate::section_0440::scan_int;
    use crate::section_1050::report_illegal_case;
    use crate::section_1091::norm_min;
    use crate::section_1341::language_node;
    use crate::section_1341::what_lang;
    use crate::section_1341::what_lhm;
    use crate::section_1341::what_rhm;
    use crate::section_1349::new_whatsit;
}}
