//! @ \TeX\ is not always in vertical mode at the time |build_page|
//! is called; the current mode reflects what \TeX\ should return to, after
//! the contribution list has been emptied. A call on |build_page| should
//! be immediately followed by `|goto big_switch|', which is \TeX's central
//! control point.
//
// @d contribute=80 {go here to link a node into the current page}
//
// @p @t\4@>@<Declare the procedure called |fire_up|@>@;@/
// procedure build_page; {append contributions to the current page}
/// append contributions to the current page
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn build_page(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit,done,done1,continue,contribute,update_heights;
    // var p:pointer; {the node being appended}
    /// the node being appended
    let mut p: pointer;
    // @!q,@!r:pointer; {nodes being examined}
    // @!b,@!c:integer; {badness and cost of current page}
    // @!pi:integer; {penalty to be added to the badness}
    const _ : () = ();
    // @!n:min_quarterword..255; {insertion box number}
    // @!delta,@!h,@!w:scaled; {sizes used for insertion calculations}
    // begin if (link(contrib_head)=null)or output_active then return;
    if link!(globals, contrib_head) == null || globals.output_active {
        return_nojump!();
    }
    // repeat continue: p:=link(contrib_head);@/
    loop {
        region_backward_label!(
        'continue_ <-
        {
        p = link!(globals, contrib_head);
        // @<Update the values of |last_glue|, |last_penalty|, and |last_kern|@>;
        Update_the_values_of_last_glue_last_penalty_and_last_kern!(globals, p);
        // @<Move node |p| to the current page; if it is time for a page break,
        //   put the nodes following the break back onto the contribution list,
        //   and |return| to the user's output routine if there is one@>;
        Move_node_p_to_the_current_page__if_it_is_time_for_a_page_break__put_the_nodes_following_the_break_back_onto_the_contribution_list__and_return_to_the_user_s_output_routine_if_there_is_one!
            (globals, p, 'continue_);
        }
        |'continue_|
        );
        // until link(contrib_head)=null;
        if link!(globals, contrib_head) == null {
            break;
        }
    }
    // @<Make the contribution list empty by setting its tail to |contrib_head|@>;
    Make_the_contribution_list_empty_by_setting_its_tail_to_contrib_head!(globals);
    // exit:end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0162::contrib_head;
