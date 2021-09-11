//! ` `
// @<Prepare to move a box or rule node to the current page...@>=
pub(crate) macro Prepare_to_move_a_box_or_rule_node_to_the_current_page__then_goto_contribute($globals:expr, $p:expr, $lbl_contribute:lifetime) {{
    // begin page_total:=page_total+page_depth+height(p);
    page_total!($globals) += page_depth!($globals) + height!($globals, $p);
    // page_depth:=depth(p);
    page_depth!($globals) = depth!($globals, $p);
    // goto contribute;
    crate::goto_forward_label!($lbl_contribute);
    // end
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0982::page_depth;
    use crate::section_0982::page_total;
}}
