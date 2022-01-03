//! ` `

// @<Prepare to move whatsit |p| to the current page, then |goto contribute|@>=
pub(crate) macro Prepare_to_move_whatsit_p_to_the_current_page__then_goto_contribute($globals:expr, $p:expr, $lbl_contribute:lifetime) {{
    // goto contribute
    crate::goto_forward_label!($lbl_contribute);
}}
