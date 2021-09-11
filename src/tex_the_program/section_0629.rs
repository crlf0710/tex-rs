//! @ The |vlist_out| routine is similar to |hlist_out|, but a bit simpler.
//
// @p procedure vlist_out; {output a |vlist_node| box}
/// output a `vlist_node` box
#[allow(unused_variables, unused_assignments)]
pub(crate) fn vlist_out(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label move_past, fin_rule, next_p;
    // var left_edge: scaled; {the left coordinate for this box}
    /// the left coordinate for this box
    let left_edge: scaled;
    // @!top_edge: scaled; {the top coordinate for this box}
    /// the top coordinate for this box
    let top_edge: scaled;
    // @!save_h,@!save_v: scaled; {what |dvi_h| and |dvi_v| should pop to}
    // @!this_box: pointer; {pointer to containing box}
    /// pointer to containing box
    let this_box: pointer;
    // @!g_order: glue_ord; {applicable order of infinity for glue}
    /// applicable order of infinity for glue
    let g_order: glue_ord;
    // @!g_sign: normal..shrinking; {selects type of glue}
    /// selects type of glue
    let g_sign: glue_sign;
    // @!p:pointer; {current position in the vlist}
    /// current position in the vlist
    let mut p: pointer;
    // @!save_loc:integer; {\.{DVI} byte location upon entry}
    /// `DVI` byte location upon entry
    let save_loc: integer;
    // @!leader_box:pointer; {the leader box being replicated}
    // @!leader_ht:scaled; {height of leader box being replicated}
    // @!lx:scaled; {extra space between leader boxes}
    // @!outer_doing_leaders:boolean; {were we doing leaders?}
    // @!edge:scaled; {bottom boundary of leader space}
    // @!glue_temp:real; {glue value before rounding}
    // @!cur_glue:real; {glue seen so far}
    /// glue seen so far
    let mut cur_glue: real;
    // @!cur_g:scaled; {rounded equivalent of |cur_glue| times the glue ratio}
    /// rounded equivalent of `cur_glue` times the glue ratio
    let mut cur_g: scaled;
    // begin cur_g:=0; cur_glue:=float_constant(0);
    cur_g = scaled::zero();
    cur_glue = float_constant!(0);
    // this_box:=temp_ptr; g_order:=glue_order(this_box);
    this_box = globals.temp_ptr;
    g_order = glue_order!(globals, this_box).into();
    // g_sign:=glue_sign(this_box); p:=list_ptr(this_box);
    g_sign = glue_sign!(globals, this_box).into();
    p = list_ptr!(globals, this_box);
    // incr(cur_s);
    incr!(globals.cur_s);
    // if cur_s>0 then dvi_out(push);
    if globals.cur_s > 0 {
        dvi_out!(globals, push.byte());
    }
    // if cur_s>max_push then max_push:=cur_s;
    if globals.cur_s > globals.max_push {
        globals.max_push = globals.cur_s;
    }
    // save_loc:=dvi_offset+dvi_ptr; left_edge:=cur_h; cur_v:=cur_v-height(this_box);
    save_loc = globals.dvi_offset + globals.dvi_ptr.get() as integer;
    left_edge = globals.cur_h;
    globals.cur_v -= height!(globals, this_box);
    // top_edge:=cur_v;
    top_edge = globals.cur_v;
    // while p<>null do @<Output node |p| for |vlist_out| and move to the next node,
    //   maintaining the condition |cur_h=left_edge|@>;
    while p != null {
        crate::section_0630::Output_node_p_for_vlist_out_and_move_to_the_next_node__maintaining_the_condition_cur_h_eq_left_edge!(
            globals, p, left_edge, this_box, cur_glue, cur_g, g_sign, g_order
        );
    }
    // prune_movements(save_loc);
    prune_movements(globals, save_loc);
    // if cur_s>0 then dvi_pop(save_loc);
    if globals.cur_s > 0 {
        dvi_pop(globals, save_loc);
    }
    // decr(cur_s);
    decr!(globals.cur_s);
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::pascal::real;
use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0016::incr;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0109::float_constant;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0135::glue_order;
use crate::section_0135::glue_sign;
use crate::section_0135::height;
use crate::section_0135::list_ptr;
use crate::section_0150::glue_ord;
use crate::section_0586::push;
use crate::section_0598::dvi_out;
use crate::section_0601::dvi_pop;
use crate::section_0615::prune_movements;
