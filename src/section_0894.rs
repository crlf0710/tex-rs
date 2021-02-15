//! @ When the following code is activated, the |line_break| procedure is in its
//! second pass, and |cur_p| points to a glue node.
//
// @<Try to hyphenate...@>=
macro_rules! Try_to_hyphenate_the_following_word {
    ($globals:expr) => {{
        /// miscellaneous nodes of temporary interest
        let (s, prev_s): (pointer, pointer);
        // begin prev_s:=cur_p; s:=link(prev_s);
        prev_s = $globals.cur_p;
        s = link!($globals, prev_s);
        // if s<>null then
        if s != null {
            // begin @<Skip to node |ha|, or |goto done1| if no hyphenation
            //   should be attempted@>;
            // if l_hyf+r_hyf>63 then goto done1;
            // @<Skip to node |hb|, putting letters into |hu| and |hc|@>;
            // @<Check that the nodes following |hb| permit hyphenation and that at least
            //   |l_hyf+r_hyf| letters have been found, otherwise |goto done1|@>;
            // hyphenate;
            // end;
            todo!("hyph");
        }
        // done1: end
    }}
}
