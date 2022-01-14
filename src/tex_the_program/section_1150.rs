//! @ Subformulas of math formulas cause a new level of math mode to be entered,
//! on the semantic nest as well as the save stack. These subformulas arise in
//! several ways: (1)~A left brace by itself indicates the beginning of a
//! subformula that will be put into a box, thereby freezing its glue and
//! preventing line breaks. (2)~A subscript or superscript is treated as a
//! subformula if it is not a single character; the same applies to
//! the nucleus of things like \.{\\underline}. (3)~The \.{\\left} primitive
//! initiates a subformula that will be terminated by a matching \.{\\right}.
//! The group codes placed on |save_stack| in these three cases are
//! |math_group|, |math_group|, and |math_left_group|, respectively.
//!
//! Here is the code that handles case (1); the other cases are not quite as
//! trivial, so we shall consider them later.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1150($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+left_brace: begin tail_append(new_noad);
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + left_brace as u16 {
        tail_append!($globals, new_noad($globals)?);
        // back_input; scan_math(nucleus(tail));
        back_input($globals);
        scan_math($globals, nucleus!(tail!($globals)))?;
        // end;
        true
    } else {
        false
    };
    use crate::section_0207::left_brace;
    use crate::section_0211::mmode;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0325::back_input;
    use crate::section_0681::nucleus;
    use crate::section_0686::new_noad;
    use crate::section_1151::scan_math;
    processed
}}
