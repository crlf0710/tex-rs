//! @ Subscripts and superscripts are attached to the previous nucleus by the
//! @^superscripts@>@^subscripts@>
//! action procedure called |sub_sup|. We use the facts that |sub_mark=sup_mark+1|
//! and |subscr(p)=supscr(p)+1|.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1175($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+sub_mark,mmode+sup_mark: sub_sup;
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + sub_mark as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + sup_mark as u16
    {
        sub_sup($globals)?;
        true
    } else {
        false
    };
    use crate::section_0207::*;
    use crate::section_0211::*;
    use crate::section_1176::sub_sup;
    processed
}}
