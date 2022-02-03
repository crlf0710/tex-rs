//! ` `
// @<Change the current style and |goto delete_q|@>=
pub(crate) macro Change_the_current_style_and_goto_delete_q($globals:expr, $q:expr, $s:expr, $lbl_delete_q:lifetime) {{
    // begin cur_style:=subtype(q); s:=style_node_size;
    $globals.cur_style = subtype!($globals, $q).into();
    $s = style_node_size;
    // @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
    crate::section_0703::Set_up_the_values_of_cur_size_and_cur_mu__based_on_cur_style!($globals);
    // goto delete_q;
    crate::goto_forward_label!($lbl_delete_q);
    // end
    use crate::section_0133::subtype;
    use crate::section_0688::style_node_size;
}}
