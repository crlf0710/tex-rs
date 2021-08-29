//! @ When the following code is activated, the |line_break| procedure is in its
//! second pass, and |cur_p| points to a glue node.
//
// @<Try to hyphenate...@>=
macro_rules! Try_to_hyphenate_the_following_word {
    ($globals:expr) => {{
        /// miscellaneous nodes of temporary interest
        let (mut s, mut prev_s): (pointer, pointer);
        // begin prev_s:=cur_p; s:=link(prev_s);
        prev_s = $globals.cur_p;
        s = link!($globals, prev_s);
        region_forward_label!(
        |'done1|
        {
        // if s<>null then
        if s != null {
            // begin @<Skip to node |ha|, or |goto done1| if no hyphenation
            //   should be attempted@>;
            Skip_to_node_ha__or_goto_done1_if_no_hyphenation_should_be_attempted!($globals, s, prev_s, 'done1);
            // if l_hyf+r_hyf>63 then goto done1;
            if $globals.l_hyf + $globals.r_hyf > 63 {
                goto_forward_label!('done1);
            }
            // @<Skip to node |hb|, putting letters into |hu| and |hc|@>;
            Skip_to_node_hb__putting_letters_into_hu_and_hc!($globals, s);
            // @<Check that the nodes following |hb| permit hyphenation and that at least
            //   |l_hyf+r_hyf| letters have been found, otherwise |goto done1|@>;
            Check_that_the_nodes_following_hb_permit_hyphenation_and_that_at_least_l_hyf_plus_r_hyf_letters_have_been_found__otherwise_goto_done1!
                ($globals, s, 'done1);
            // hyphenate;
            hyphenate($globals);
            // end;
        }
        }
        // done1: end
        'done1 <-
        );
        use crate::section_0895::hyphenate;
    }}
}
