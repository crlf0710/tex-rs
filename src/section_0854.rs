//! @ During the final pass, we dare not lose all active nodes, lest we lose
//! touch with the line breaks already found. The code shown here makes sure
//! that such a catastrophe does not happen, by permitting overfull boxes as
//! a last resort. This particular part of \TeX\ was a source of several subtle
//! bugs before the correct program logic was finally discovered; readers
//! who seek to ``improve'' \TeX\ should therefore think thrice before daring
//! to make any changes here.
//! @^overfull boxes@>
//
// @<Prepare to deactivate node~|r|, and |goto deactivate| unless...@>=
macro_rules! Prepare_to_deactivate_node_r__and_goto_deactivate_unless_there_is_a_reason_to_consider_lines_of_text_from_r_to_cur_p {
    ($globals:expr, $b:expr, $r:expr, $prev_r:expr, $artificial_demerits:expr, $node_r_stays_active:expr, $lbl_deactivate:lifetime) => {{
        // begin if final_pass and (minimum_demerits=awful_bad) and@|
        //    (link(r)=last_active) and
        //    (prev_r=active) then
        if $globals.final_pass
            && $globals.minimum_demerits == awful_bad
            && link!($globals, $r) == last_active!()
            && $prev_r == active
        {
            // artificial_demerits:=true {set demerits zero, this break is forced}
            /// set demerits zero, this break is forced
            const _: () = ();
            $artificial_demerits = true;
        }
        // else if b>threshold then goto deactivate;
        else if $b as integer > $globals.threshold {
            goto_forward_label!($lbl_deactivate);
        }
        // node_r_stays_active:=false;
        $node_r_stays_active = false;
        // end
        use crate::section_0833::awful_bad;
    }};
}
