//! @ Now we are ready to consider |vsplit| itself. Most of
//! its work is accomplished by the two subroutines that we have just considered.
//!
//! Given the number of a vlist box |n|, and given a desired page height |h|,
//! the |vsplit| function finds the best initial segment of the vlist and
//! returns a box for a page of height~|h|. The remainder of the vlist, if
//! any, replaces the original box, after removing glue and penalties and
//! adjusting for |split_top_skip|. Mark nodes in the split-off box are used to
//! set the values of |split_first_mark| and |split_bot_mark|; we use the
//! fact that |split_first_mark=null| if and only if |split_bot_mark=null|.
//!
//! The original box becomes ``void'' if and only if it has been entirely
//! extracted.  The extracted box is ``void'' if and only if the original
//! box was void (or if it was, erroneously, an hlist box).
//
// @p function vsplit(@!n:eight_bits; @!h:scaled):pointer;
//   {extracts a page of height |h| from box |n|}
/// extracts a page of height `h` from box `n`
#[allow(unused_variables, unused_assignments)]
pub(crate) fn vsplit(globals: &mut TeXGlobals, n: eight_bits, h: scaled) -> TeXResult<pointer> {
    // label exit,done;
    // var v:pointer; {the box to be split}
    /// the box to be split
    let v;
    // p:pointer; {runs through the vlist}
    /// runs through the vlist
    let mut p;
    // q:pointer; {points to where the break occurs}
    /// points to where the break occurs
    let mut q;
    // begin v:=box(n);
    v = r#box!(globals, n);
    // if split_first_mark<>null then
    if split_first_mark!(globals) != null {
        // begin delete_token_ref(split_first_mark); split_first_mark:=null;
        // delete_token_ref(split_bot_mark); split_bot_mark:=null;
        // end;
        todo!("split_first_mark != null");
    }
    // @<Dispense with trivial cases of void or bad boxes@>;
    crate::section_0978::Dispense_with_trivial_cases_of_void_or_bad_boxes!(globals, v);
    // q:=vert_break(list_ptr(v),h,split_max_depth);
    q = vert_break(globals, list_ptr!(globals, v), h, split_max_depth!(globals))?;
    // @<Look at all the marks in nodes before the break, and set the final
    //   link to |null| at the break@>;
    crate::section_0979::Look_at_all_the_marks_in_nodes_before_the_break__and_set_the_final_link_to_null_at_the_break!(
        globals, p, q, v
    );
    // q:=prune_page_top(q); p:=list_ptr(v); free_node(v,box_node_size);
    q = prune_page_top(globals, q)?;
    p = list_ptr!(globals, v);
    free_node(globals, v, box_node_size as _);
    // if q=null then box(n):=null {the |eq_level| of the box stays the same}
    if q == null {
        r#box!(globals, n) = null;
        /// the `eq_level` of the box stays the same
        const _: () = ();
    }
    // else box(n):=vpack(q,natural);
    else {
        r#box!(globals, n) = vpack(globals, q, natural0!(), natural1!())?;
    }
    // vsplit:=vpackage(p,h,exactly,split_max_depth);
    let vsplit = vpackage(globals, p, h, exactly.into(), split_max_depth!(globals))?;
    // exit: end;
    crate::ok_nojump!(vsplit)
}

use crate::section_0004::TeXGlobals;
use crate::section_0025::eight_bits;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0130::free_node;
use crate::section_0135::box_node_size;
use crate::section_0135::list_ptr;
use crate::section_0137::vlist_node;
use crate::section_0230::r#box;
use crate::section_0247::split_max_depth;
use crate::section_0382::split_first_mark;
use crate::section_0644::exactly;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0668::vpack;
use crate::section_0668::vpackage;
use crate::section_0968::prune_page_top;
use crate::section_0970::vert_break;
