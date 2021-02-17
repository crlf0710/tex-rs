//! @ The |vlist_out| routine is similar to |hlist_out|, but a bit simpler.
//
// @p procedure vlist_out; {output a |vlist_node| box}
/// output a `vlist_node` box
#[allow(unused_variables)]
pub(crate) fn vlist_out(globals: &mut TeXGlobals) {
    // label move_past, fin_rule, next_p;
    // var left_edge: scaled; {the left coordinate for this box}
    // @!top_edge: scaled; {the top coordinate for this box}
    // @!save_h,@!save_v: scaled; {what |dvi_h| and |dvi_v| should pop to}
    // @!this_box: pointer; {pointer to containing box}
    // @!g_order: glue_ord; {applicable order of infinity for glue}
    // @!g_sign: normal..shrinking; {selects type of glue}
    // @!p:pointer; {current position in the vlist}
    // @!save_loc:integer; {\.{DVI} byte location upon entry}
    // @!leader_box:pointer; {the leader box being replicated}
    // @!leader_ht:scaled; {height of leader box being replicated}
    // @!lx:scaled; {extra space between leader boxes}
    // @!outer_doing_leaders:boolean; {were we doing leaders?}
    // @!edge:scaled; {bottom boundary of leader space}
    // @!glue_temp:real; {glue value before rounding}
    // @!cur_glue:real; {glue seen so far}
    // @!cur_g:scaled; {rounded equivalent of |cur_glue| times the glue ratio}
    // begin cur_g:=0; cur_glue:=float_constant(0);
    // this_box:=temp_ptr; g_order:=glue_order(this_box);
    // g_sign:=glue_sign(this_box); p:=list_ptr(this_box);
    // incr(cur_s);
    // if cur_s>0 then dvi_out(push);
    // if cur_s>max_push then max_push:=cur_s;
    // save_loc:=dvi_offset+dvi_ptr; left_edge:=cur_h; cur_v:=cur_v-height(this_box);
    // top_edge:=cur_v;
    // while p<>null do @<Output node |p| for |vlist_out| and move to the next node,
    //   maintaining the condition |cur_h=left_edge|@>;
    // prune_movements(save_loc);
    // if cur_s>0 then dvi_pop(save_loc);
    // decr(cur_s);
    // end;
}

use crate::section_0004::TeXGlobals;
