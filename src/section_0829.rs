//! @ The heart of the line-breaking procedure is `|try_break|', a subroutine
//! that tests if the current breakpoint |cur_p| is feasible, by running
//! through the active list to see what lines of text can be made from active
//! nodes to~|cur_p|.  If feasible breaks are possible, new break nodes are
//! created.  If |cur_p| is too far from an active node, that node is
//! deactivated.
//!
//! The parameter |pi| to |try_break| is the penalty associated
//! with a break at |cur_p|; we have |pi=eject_penalty| if the break is forced,
//! and |pi=inf_penalty| if the break is illegal.
//!
//! The other parameter, |break_type|, is set to |hyphenated| or |unhyphenated|,
//! depending on whether or not the current break is at a |disc_node|. The
//! end of a paragraph is also regarded as `|hyphenated|'; this case is
//! distinguishable by the condition |cur_p=null|.
//
// @d copy_to_cur_active(#)==cur_active_width[#]:=active_width[#]
macro_rules! copy_to_cur_active {
    ($globals:expr, $idx:expr) => {{
        $globals.cur_active_width[$idx] = $globals.active_width[$idx];
    }}
}
// @d deactivate=60 {go here when node |r| should be deactivated}
//
// @<Declare subprocedures for |line_break|@>=
// procedure try_break(@!pi:integer;@!break_type:small_number);
#[allow(unused_variables, unused_assignments)]
pub(crate) fn try_break(globals: &mut TeXGlobals, mut pi: integer, break_type: small_number) -> TeXResult<()> {
    // label exit,done,done1,continue,deactivate;
    // var r:pointer; {runs through the active list}
    /// runs through the active list
    let mut r: pointer;
    // @!prev_r:pointer; {stays a step behind |r|}
    /// stays a step behind `r`
    let mut prev_r: pointer;
    // @!old_l:halfword; {maximum line number in current equivalence class of lines}
    /// maximum line number in current equivalence class of lines
    let mut old_l: halfword;
    // @!no_break_yet:boolean; {have we found a feasible break at |cur_p|?}
    /// have we found a feasible break at `cur_p`?
    let mut no_break_yet: boolean;
    // @<Other local variables for |try_break|@>@;
    const _ : () = ();
    // begin @<Make sure that |pi| is in the proper range@>;
    Make_sure_that_pi_is_in_the_proper_range!(globals, pi);
    // no_break_yet:=true; prev_r:=active; old_l:=0;
    no_break_yet = true;
    prev_r = active;
    old_l = 0;
    // do_all_six(copy_to_cur_active);
    do_all_six!(copy_to_cur_active !; @globals = globals);
    // loop@+  begin continue: r:=link(prev_r);
    loop {
        /// a step behind `prev_r`, if `type(prev_r)=delta_node`
        let mut prev_prev_r: pointer;
        /// the current line will be justified to this width
        let mut line_width: scaled = scaled::zero();

        region_backward_label!(
        'continue_ <-
        {
        r = link!(globals, prev_r);
        // @<If node |r| is of type |delta_node|, update |cur_active_width|,
        //   set |prev_r| and |prev_prev_r|, then |goto continue|@>;
        If_node_r_is_of_type_delta_node__update_cur_active_width__set_prev_r_and_prev_prev_r__then_goto_continue!
            (globals, r, prev_r, prev_prev_r, 'continue_);
        // @<If a line number class has ended, create new active nodes for
        //   the best feasible breaks in that class; then |return|
        //   if |r=last_active|, otherwise compute the new |line_width|@>;
        If_a_line_number_class_has_ended__create_new_active_nodes_for_the_best_feasible_breaks_in_that_class__then_return_if_r_eq_last_active__otherwise_compute_the_new_line_width!
            (globals, r, prev_r, prev_prev_r, old_l, line_width, break_type, no_break_yet);
        // @<Consider the demerits for a line from |r| to |cur_p|;
        //   deactivate node |r| if it should no longer be active;
        //   then |goto continue| if a line from |r| to |cur_p| is infeasible,
        //   otherwise record a new feasible break@>;
        Consider_the_demerits_for_a_line_from_r_to_cur_p__deactivate_node_r_if_it_should_no_longer_be_active__then_goto_continue_if_a_line_from_r_to_cur_p_is_infeasible__otherwise_record_a_new_feasible_break!
            (globals, r, prev_r, line_width, pi, break_type, 'continue_);
        // end;
        }
        |'continue_|
        );
    }
    // exit: @!stat @<Update the value of |printed_node| for
    //   symbolic displays@>@+tats@;
    region_stat! {
        todo!();
    }
    // end;
    ok_nojump!()
}

use crate::pascal::integer;
use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0101::scaled;
use crate::section_0113::halfword;
use crate::section_0115::pointer;
use crate::section_0162::active;