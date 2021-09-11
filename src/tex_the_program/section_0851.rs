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
// @<Consider the demerits for a line from |r| to |cur_p|...@>=
pub(crate) macro Consider_the_demerits_for_a_line_from_r_to_cur_p__deactivate_node_r_if_it_should_no_longer_be_active__then_goto_continue_if_a_line_from_r_to_cur_p_is_infeasible__otherwise_record_a_new_feasible_break
    ($globals:expr, $r:expr, $prev_r:expr, $prev_prev_r:expr, $l:expr, $line_width:expr, $pi:expr, $break_type:expr, $lbl_continue:lifetime) {{
        crate::trace_span!("Consider the demerits for a line from |r| to |cur_p|...");
        /// has `d` been forced to zero?
        let mut artificial_demerits: boolean;
        /// used in badness calculations
        let shortfall: scaled;
        /// should node `r` remain in the active list?
        let node_r_stays_active: boolean;
        /// badness of test line
        let b: halfword;
        /// possible fitness class of test line
        let fit_class: fit_class_kind;

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
            crate::section_0852::Set_the_value_of_b_to_the_badness_for_stretching_the_line__and_compute_the_corresponding_fit_class!
                ($globals, b, fit_class, shortfall);
        }
        // else @<Set the value of |b| to the badness for shrinking the line,
        //     and compute the corresponding |fit_class|@>;
        else {
            crate::section_0853::Set_the_value_of_b_to_the_badness_for_shrinking_the_line__and_compute_the_corresponding_fit_class!
                ($globals, b, fit_class, shortfall);
        }
        crate::region_forward_label!(
        |'deactivate|
        {
        // if (b>inf_bad)or(pi=eject_penalty) then
        if b > inf_bad || $pi == eject_penalty {
            // @<Prepare to deactivate node~|r|, and |goto deactivate| unless
            //   there is a reason to consider lines of text from |r| to |cur_p|@>
            crate::section_0854::Prepare_to_deactivate_node_r__and_goto_deactivate_unless_there_is_a_reason_to_consider_lines_of_text_from_r_to_cur_p!
                ($globals, b, $r, $prev_r, artificial_demerits, node_r_stays_active, 'deactivate);
        }
        // else  begin prev_r:=r;
        else {
            $prev_r = $r;
            // if b>threshold then goto continue;
            if b as integer > $globals.threshold {
                crate::trace_span!("jump because b > threshold");
                crate::goto_backward_label!($lbl_continue);
            }
            // node_r_stays_active:=true;
            node_r_stays_active = true;
            // end;
        }
        // @<Record a new feasible break@>;
        crate::section_0855::Record_a_new_feasible_break!($globals, $r, $l, b, $pi, $break_type, fit_class, artificial_demerits);
        // if node_r_stays_active then goto continue; {|prev_r| has been set to |r|}
        if node_r_stays_active {
            crate::trace_span!("jump because node_r_stays_active");
            /// `prev_r` has been set to `r`
            crate::goto_backward_label!($lbl_continue);
        }
        // deactivate: @<Deactivate node |r|@>;
        }
        'deactivate <-
        );
        crate::section_0860::Deactivate_node_r!($globals, $r, $prev_r, $prev_prev_r);
        // end
        use crate::pascal::boolean;
        use crate::pascal::integer;
        use crate::section_0101::scaled;
        use crate::section_0108::inf_bad;
        use crate::section_0113::halfword;
        use crate::section_0157::eject_penalty;
        use crate::section_0817::fit_class_kind;
    }}
