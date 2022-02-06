//! @ The next subroutine finds the best place to break a given vertical list
//! so as to obtain a box of height~|h|, with maximum depth~|d|.
//! A pointer to the beginning of the vertical list is given,
//! and a pointer to the optimum breakpoint is returned. The list is effectively
//! followed by a forced break, i.e., a penalty node with the |eject_penalty|;
//! if the best break occurs at this artificial node, the value |null| is returned.
//!
//! An array of six |scaled| distances is used to keep track of the height
//! from the beginning of the list to the current place, just as in |line_break|.
//! In fact, we use one of the same arrays, only changing its name to reflect
//! its new significance.
//
// @d active_height==active_width {new name for the six distance variables}
/// new name for the six distance variables
pub(crate) macro active_height($globals:expr) {
    $globals.active_width
}
// @d cur_height==active_height[1] {the natural height}
/// the natural height
pub(crate) macro cur_height($globals:expr) {
    active_height!($globals)[1]
}
// @d set_height_zero(#)==active_height[#]:=0 {initialize the height to zero}
/// initialize the height to zero
pub(self) macro set_height_zero($globals:expr, $v:expr) {
    active_height!($globals)[$v] = crate::section_0101::scaled::zero();
}
// @#
// @d update_heights=90 {go here to record glue in the |active_height| table}
//
// @p function vert_break(@!p:pointer; @!h,@!d:scaled):pointer;
//   {finds optimum page break}
/// finds optimum page break
#[allow(unused_assignments, unused_variables)]
pub(crate) fn vert_break(
    globals: &mut TeXGlobals,
    mut p: pointer,
    h: scaled,
    d: scaled,
) -> TeXResult<pointer> {
    // label done,not_found,update_heights;
    // var prev_p:pointer; {if |p| is a glue node, |type(prev_p)| determines
    //   whether |p| is a legal breakpoint}
    /// if `p` is a glue node, `type(prev_p)` determines whether `p` is a legal breakpoint
    let mut prev_p;
    // @!q,@!r:pointer; {glue specifications}
    // @!pi:integer; {penalty value}
    /// penalty value
    let mut pi;
    // @!b:integer; {badness at a trial breakpoint}
    // @!least_cost:integer; {the smallest badness plus penalties found so far}
    /// the smallest badness plus penalties found so far
    let mut least_cost;
    // @!best_place:pointer; {the most recent break that leads to |least_cost|}
    /// the most recent break that leads to `least_cost`
    let mut best_place = null;
    // @!prev_dp:scaled; {depth of previous box in the list}
    /// depth of previous box in the list
    let mut prev_dp;
    // @!t:small_number; {|type| of the node following a kern}
    // begin prev_p:=p; {an initial glue node is not a legal breakpoint}
    /// an initial glue node is not a legal breakpoint
    const _: () = ();
    prev_p = p;
    // least_cost:=awful_bad; do_all_six(set_height_zero); prev_dp:=0;
    least_cost = awful_bad;
    do_all_six!(set_height_zero !; @globals = globals);
    prev_dp = scaled::zero();
    crate::region_forward_label! {
        |'done|
        {
            // loop@+  begin @<If node |p| is a legal breakpoint, check if this break is
            //     the best known, and |goto done| if |p| is null or
            //     if the page-so-far is already too full to accept more stuff@>;
            loop {
                crate::section_0972::If_node_p_is_a_legal_breakpoint__check_if_this_break_is_the_best_known__and_goto_done_if_p_is_null_or_if_the_page_so_far_is_already_too_full_to_accept_more_stuff!(
                    globals, p, h, d, pi, prev_p, prev_dp, best_place, least_cost, 'done
                );
                // prev_p:=p; p:=link(prev_p);
                prev_p = p;
                p = link!(globals, prev_p);
                // end;
            }
        }
        // done: vert_break:=best_place;
        'done <-
    }
    let vert_break = best_place;
    // end;
    crate::ok_nojump!(vert_break)
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0823::do_all_six;
use crate::section_0833::awful_bad;
