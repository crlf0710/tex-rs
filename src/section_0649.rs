//! @ Here now is |hpack|, which contains few if any surprises.
//
// @p function hpack(@!p:pointer;@!w:scaled;@!m:small_number):pointer;
#[allow(unused_variables)]
pub(crate) fn hpack(globals: &mut TeXGlobals, mut p: pointer, mut w: scaled, m: small_number) -> TeXResult<pointer> {
    // label reswitch, common_ending, exit;
    // var r:pointer; {the box node that will be returned}
    /// the box node that will be returned
    let r: pointer;
    // @!q:pointer; {trails behind |p|}
    /// trails behind `p`
    let q: pointer;
    // @!h,@!d,@!x:scaled; {height, depth, and natural width}
    /// height, depth, and natural width
    let (mut h, mut d, mut x): (scaled, scaled, scaled);
    // @!s:scaled; {shift amount}
    // @!g:pointer; {points to a glue specification}
    // @!o:glue_ord; {order of infinity}
    // @!f:internal_font_number; {the font in a |char_node|}
    // @!i:four_quarters; {font information about a |char_node|}
    // @!hd:eight_bits; {height and depth indices for a character}
    // begin last_badness:=0; r:=get_node(box_node_size); type(r):=hlist_node;
    globals.last_badness = 0;
    r = get_node(globals, box_node_size as _)?;
    r#type!(globals, r) = hlist_node;
    // subtype(r):=min_quarterword; shift_amount(r):=0;
    subtype!(globals, r) = min_quarterword;
    shift_amount!(globals, r) = scaled::zero();
    // q:=r+list_offset; link(q):=p;@/
    q = r + list_offset as pointer;
    link!(globals, q) = p;
    // h:=0; @<Clear dimensions to zero@>;
    h = scaled::zero();
    Clear_dimensions_to_zero!(globals, d, x);
    // while p<>null do @<Examine node |p| in the hlist, taking account of its effect
    //   on the dimensions of the new box, or moving it to the adjustment list;
    //   then advance |p| to the next node@>;
    while p != null {
        Examine_node_p_in_the_hlist__taking_account_of_its_effect_on_the_dimensions_of_the_new_box__or_moving_it_to_the_adjustment_list__then_advance_p_to_the_next_node!
            (globals, p, h, d, x);
    }
    // if adjust_tail<>null then link(adjust_tail):=null;
    if globals.adjust_tail != null {
        link!(globals, globals.adjust_tail) = null;
    }
    // height(r):=h; depth(r):=d;@/
    height!(globals, r) = h;
    depth!(globals, r) = d;
    // @<Determine the value of |width(r)| and the appropriate glue setting;
    //   then |return| or |goto common_ending|@>;
    Determine_the_value_of_width_r_and_the_appropriate_glue_setting__then_return_or_goto_common_ending!
        (globals, m, r, w, x);
    // common_ending: @<Finish issuing a diagnostic message
    //       for an overfull or underfull hbox@>;
    Finish_issuing_a_diagnostic_message_for_an_overfull_or_underfull_hbox!(globals);
    // exit: hpack:=r;
    todo!("hpack");
    return_nojump!(r);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0110::min_quarterword;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0125::get_node;
use crate::section_0135::hlist_node;
use crate::section_0135::box_node_size;
use crate::section_0135::list_offset;
