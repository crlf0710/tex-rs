//! ` `
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1158($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+math_comp: begin tail_append(new_noad);
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + math_shift as u16 {
        tail_append!($globals, new_noad($globals)?);
        // type(tail):=cur_chr; scan_math(nucleus(tail));
        r#type!($globals, tail!($globals)) = $globals.cur_chr.get() as _;
        scan_math($globals, nucleus!(tail!($globals)))?;
        // end;
        true
    }
    // mmode+limit_switch: math_limit_switch;
    else if $abs_mode_plus_cur_cmd == mmode as u16 + limit_switch as u16 {
        todo!("math_limit_switch");
        true
    } else {
        false
    };
    use crate::section_0133::r#type;
    use crate::section_0207::*;
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0681::nucleus;
    use crate::section_0686::new_noad;
    use crate::section_1151::scan_math;
    processed
}}
