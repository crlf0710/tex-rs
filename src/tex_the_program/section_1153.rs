//! @ The pointer |p| is placed on |save_stack| while a complex subformula
//! is being scanned.
//
// @<Scan a subformula...@>=
pub(crate) macro Scan_a_subformula_enclosed_in_braces_and_return($globals:expr, $p:expr) {{
    // begin back_input; scan_left_brace;@/
    back_input($globals);
    scan_left_brace($globals)?;
    // saved(0):=p; incr(save_ptr); push_math(math_group); return;
    saved!($globals, 0) = $p as _;
    incr!($globals.save_ptr);
    push_math($globals, math_group.into());
    crate::return_nojump!();
    // end
    use crate::section_0016::incr;
    use crate::section_0269::math_group;
    use crate::section_0274::saved;
    use crate::section_0325::back_input;
    use crate::section_0403::scan_left_brace;
    use crate::section_1136::push_math;
}}
