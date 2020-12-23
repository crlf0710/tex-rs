//! @ Here now is |hpack|, which contains few if any surprises.
//
// @p function hpack(@!p:pointer;@!w:scaled;@!m:small_number):pointer;
#[allow(unused_variables)]
pub(crate) fn hpack(globals: &mut TeXGlobals, p: pointer, w: scaled, m: small_number) -> pointer {
    // label reswitch, common_ending, exit;
    // var r:pointer; {the box node that will be returned}
    /// the box node that will be returned
    let r: pointer;
    // @!q:pointer; {trails behind |p|}
    // @!h,@!d,@!x:scaled; {height, depth, and natural width}
    // @!s:scaled; {shift amount}
    // @!g:pointer; {points to a glue specification}
    // @!o:glue_ord; {order of infinity}
    // @!f:internal_font_number; {the font in a |char_node|}
    // @!i:four_quarters; {font information about a |char_node|}
    // @!hd:eight_bits; {height and depth indices for a character}
    // begin last_badness:=0; r:=get_node(box_node_size); type(r):=hlist_node;
    // subtype(r):=min_quarterword; shift_amount(r):=0;
    // q:=r+list_offset; link(q):=p;@/
    // h:=0; @<Clear dimensions to zero@>;
    // while p<>null do @<Examine node |p| in the hlist, taking account of its effect
    //   on the dimensions of the new box, or moving it to the adjustment list;
    //   then advance |p| to the next node@>;
    // if adjust_tail<>null then link(adjust_tail):=null;
    // height(r):=h; depth(r):=d;@/
    // @<Determine the value of |width(r)| and the appropriate glue setting;
    //   then |return| or |goto common_ending|@>;
    // common_ending: @<Finish issuing a diagnostic message
    //       for an overfull or underfull hbox@>;
    // exit: hpack:=r;
    todo!("hpack");
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0115::pointer;