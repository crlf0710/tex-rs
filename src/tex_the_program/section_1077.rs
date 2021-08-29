//! ` `

// @<Store \(c)|cur_box| in a box register@>=
macro_rules! Store_cur_box_in_a_box_register {
    ($globals:expr, $box_context:expr) => {{
        // if box_context<box_flag+256 then
        if $box_context < box_flag + 256 {
            // eq_define(box_base-box_flag+box_context,box_ref,cur_box)
            eq_define(
                $globals,
                box_base as pointer + ($box_context - box_flag) as pointer,
                box_ref,
                $globals.cur_box,
            )?;
        }
        // else geq_define(box_base-box_flag-256+box_context,box_ref,cur_box)
        else {
            geq_define(
                $globals,
                box_base as pointer + ($box_context - box_flag - 256) as pointer,
                box_ref,
                $globals.cur_box,
            )?;
        }

        use crate::section_0115::pointer;
        use crate::section_0210::box_ref;
        use crate::section_0230::box_base;
        use crate::section_0277::eq_define;
        use crate::section_0279::geq_define;
    }};
}
