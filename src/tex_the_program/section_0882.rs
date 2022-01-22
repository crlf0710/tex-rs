//! ` `
// @<Change discretionary to compulsory...@>=
pub(crate) macro Change_discretionary_to_compulsory_and_set_disc_break_to_true($globals:expr, $q:expr, $disc_break:expr, $post_disc_break:expr) {{
    /// used for replacement counts in discretionary nodes
    let mut t;
    /// temporary registers for list manipulation
    let mut r;
    // begin t:=replace_count(q);
    t = replace_count!($globals, $q);
    // @<Destroy the |t| nodes following |q|, and
    //    make |r| point to the following node@>;
    crate::section_0883::Destroy_the_t_nodes_following_q__and_make_r_point_to_the_following_node!(
        $globals, $q, r, t
    );
    // if post_break(q)<>null then @<Transplant the post-break list@>;
    if post_break!($globals, $q) != null {
        crate::section_0884::Transplant_the_post_break_list!($globals, $q, r, $post_disc_break);
    }
    // if pre_break(q)<>null then @<Transplant the pre-break list@>;
    if pre_break!($globals, $q) != null {
        crate::section_0885::Transplant_the_pre_break_list!($globals, $q);
    }
    // link(q):=r; disc_break:=true;
    link!($globals, $q) = r;
    $disc_break = true;
    // end
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0145::post_break;
    use crate::section_0145::pre_break;
    use crate::section_0145::replace_count;
}}
