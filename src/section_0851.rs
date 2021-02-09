//! @ The remaining part of |try_break| deals with the calculation of
//! demerits for a break from |r| to |cur_p|.
//!
//! The first thing to do is calculate the badness, |b|. This value will always
//! be between zero and |inf_bad+1|; the latter value occurs only in the
//! case of lines from |r| to |cur_p| that cannot shrink enough to fit the necessary
//! width. In such cases, node |r| will be deactivated.
//! We also deactivate node~|r| when a break at~|cur_p| is forced, since future
//! breaks must go through a forced break.
//
// @<Consider the demerits for a line from |r| to |cur_p|...@>atgrfdsssssd=
macro_rules! Consider_the_demerits_for_a_line_from_r_to_cur_p__deactivate_node_r_if_it_should_no_longer_be_active__then_goto_continue_if_a_line_from_r_to_cur_p_is_infeasible__otherwise_record_a_new_feasible_break {
    ($globals:expr, $r:expr, $prev_r:expr, $line_width:expr, $pi:expr, $lbl_continue:lifetime) => {{
        /// has `d` been forced to zero?
        let artificial_demerits: boolean;
        /// used in badness calculations
        let shortfall: scaled;
        /// should node `r` remain in the active list?
        let node_r_stays_active: boolean;
        /// badness of test line
        let b: halfword;

        // begin artificial_demerits:=false;@/
        artificial_demerits = false;
        // @^inner loop@>
        // shortfall:=line_width-cur_active_width[1]; {we're this much too short}
        /// we're this much too short
        const _ : () = ();
        shortfall = $line_width - $globals.cur_active_width[1];
        // if shortfall>0 then
        if shortfall > scaled::zero() {
            // @<Set the value of |b| to the badness for stretching the line,
            //   and compute the corresponding |fit_class|@>
            todo!("shortfall > 0");
        }
        // else @<Set the value of |b| to the badness for shrinking the line,
        //     and compute the corresponding |fit_class|@>;
        else {
            todo!("shortfall <= 0");
        }
        // if (b>inf_bad)or(pi=eject_penalty) then
        if b > inf_bad || $pi == eject_penalty {
            // @<Prepare to deactivate node~|r|, and |goto deactivate| unless
            //   there is a reason to consider lines of text from |r| to |cur_p|@>
            todo!("prepare to deactivate");
        }
        // else  begin prev_r:=r;
        else {
            $prev_r = $r;
            // if b>threshold then goto continue;
            if b as integer > $globals.threshold {
                goto_backward_label!($lbl_continue);
            }
            // node_r_stays_active:=true;
            node_r_stays_active = true;
            // end;
        }
        // @<Record a new feasible break@>;
        todo!("record");
        // if node_r_stays_active then goto continue; {|prev_r| has been set to |r|}
        if node_r_stays_active {
            /// `prev_r` has been set to `r`
            goto_backward_label!($lbl_continue);
        }
        // deactivate: @<Deactivate node |r|@>;
        todo!("deactivate node");
        // end
        use crate::section_0108::inf_bad;
        use crate::section_0157::eject_penalty;
    }}
}
