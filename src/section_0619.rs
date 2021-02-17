//! @ The recursive procedures |hlist_out| and |vlist_out| each have local variables
//! |save_h| and |save_v| to hold the values of |dvi_h| and |dvi_v| just before
//! entering a new level of recursion.  In effect, the values of |save_h| and
//! |save_v| on \TeX's run-time stack correspond to the values of |h| and |v|
//! that a \.{DVI}-reading program will push onto its coordinate stack.
//
// @d move_past=13 {go to this label when advancing past glue or a rule}
// @d fin_rule=14 {go to this label to finish processing a rule}
// @d next_p=15 {go to this label when finished with node |p|}
//
// @p @t\4@>@<Declare procedures needed in |hlist_out|, |vlist_out|@>@t@>@/
// procedure hlist_out; {output an |hlist_node| box}
/// output an `hlist_node` box
#[allow(unused_variables)]
pub(crate) fn hlist_out(globals: &mut TeXGlobals) {
    // label reswitch, move_past, fin_rule, next_p;
    // var base_line: scaled; {the baseline coordinate for this box}
    // @!left_edge: scaled; {the left coordinate for this box}
    // @!save_h,@!save_v: scaled; {what |dvi_h| and |dvi_v| should pop to}
    // @!this_box: pointer; {pointer to containing box}
    // @!g_order: glue_ord; {applicable order of infinity for glue}
    // @!g_sign: normal..shrinking; {selects type of glue}
    // @!p:pointer; {current position in the hlist}
    // @!save_loc:integer; {\.{DVI} byte location upon entry}
    // @!leader_box:pointer; {the leader box being replicated}
    // @!leader_wd:scaled; {width of leader box being replicated}
    // @!lx:scaled; {extra space between leader boxes}
    // @!outer_doing_leaders:boolean; {were we doing leaders?}
    // @!edge:scaled; {left edge of sub-box, or right edge of leader space}
    // @!glue_temp:real; {glue value before rounding}
    // @!cur_glue:real; {glue seen so far}
    // @!cur_g:scaled; {rounded equivalent of |cur_glue| times the glue ratio}
    // begin cur_g:=0; cur_glue:=float_constant(0);
    // this_box:=temp_ptr; g_order:=glue_order(this_box);
    // g_sign:=glue_sign(this_box); p:=list_ptr(this_box);
    // incr(cur_s);
    // if cur_s>0 then dvi_out(push);
    // if cur_s>max_push then max_push:=cur_s;
    // save_loc:=dvi_offset+dvi_ptr; base_line:=cur_v; left_edge:=cur_h;
    // while p<>null do @<Output node |p| for |hlist_out| and move to the next node,
    //   maintaining the condition |cur_v=base_line|@>;
    // prune_movements(save_loc);
    // if cur_s>0 then dvi_pop(save_loc);
    // decr(cur_s);
    // end;
    todo!();
}

use crate::section_0004::TeXGlobals;
