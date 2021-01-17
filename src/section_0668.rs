//! @ The |vpack| subroutine is actually a special case of a slightly more
//! general routine called |vpackage|, which has four parameters. The fourth
//! parameter, which is |max_dimen| in the case of |vpack|, specifies the
//! maximum depth of the page box that is constructed. The depth is first
//! computed by the normal rules; if it exceeds this limit, the reference
//! point is simply moved down until the limiting depth is attained.
//
// @d vpack(#)==vpackage(#,max_dimen) {special case of unconstrained depth}
//
// @p function vpackage(@!p:pointer;@!h:scaled;@!m:small_number;@!l:scaled):
//   pointer;
#[allow(unused_variables, unused_assignments)]
pub(crate) fn vpackage(
    globals: &mut TeXGlobals,
    p: pointer,
    h: scaled,
    m: small_number,
    l: scaled,
) -> TeXResult<pointer> {
    // label common_ending, exit;
    // var r:pointer; {the box node that will be returned}
    /// the box node that will be returned
    let r: pointer;
    // @!w,@!d,@!x:scaled; {width, depth, and natural height}
    /// width, depth, and natural height
    let (w, d, mut x): (scaled, scaled, scaled);
    // @!s:scaled; {shift amount}
    // @!g:pointer; {points to a glue specification}
    // @!o:glue_ord; {order of infinity}
    // begin last_badness:=0; r:=get_node(box_node_size); type(r):=vlist_node;
    globals.last_badness = 0;
    r = get_node(globals, box_node_size.into())?;
    r#type!(globals, r) = vlist_node;
    // subtype(r):=min_quarterword; shift_amount(r):=0;
    subtype!(globals, r) = min_quarterword;
    shift_amount!(globals, r) = scaled::zero();
    // list_ptr(r):=p;@/
    list_ptr!(globals, r) = p;
    // w:=0; @<Clear dimensions to zero@>;
    w = scaled::zero();
    Clear_dimensions_to_zero!(globals, d, x);
    // while p<>null do @<Examine node |p| in the vlist, taking account of its effect
    //   on the dimensions of the new box; then advance |p| to the next node@>;
    while p != null {
        todo!("examine and advance");
    }
    // width(r):=w;
    width!(globals, r) = w;
    // if d>l then
    if d > l {
        // begin x:=x+d-l; depth(r):=l;
        x = x + d - l;
        depth!(globals, r) = l;
    // end
    }
    // else depth(r):=d;
    else {
        depth!(globals, r) = d;
    }
    // @<Determine the value of |height(r)| and the appropriate glue setting;
    //   then |return| or |goto common_ending|@>;
    Determine_the_value_of_height_r_and_the_appropriate_glue_setting__then_return_or_goto_common_ending!(
        globals
    );
    // common_ending: @<Finish issuing a diagnostic message
    //       for an overfull or underfull vbox@>;
    Finish_issuing_a_diagnostic_message_for_an_overfull_or_underfull_vbox!(globals, r);
    // exit: vpackage:=r;
    return_nojump!(r);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0110::min_quarterword;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0135::box_node_size;
use crate::section_0137::vlist_node;
