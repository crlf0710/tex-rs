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
#[allow(unused_variables, unused_assignments)]
pub(crate) fn hlist_out(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label reswitch, move_past, fin_rule, next_p;
    // var base_line: scaled; {the baseline coordinate for this box}
    /// the baseline coordinate for this box
    let base_line: scaled;
    // @!left_edge: scaled; {the left coordinate for this box}
    /// the left coordinate for this box
    let left_edge: scaled;
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
    // @!p:pointer; {current position in the hlist}
    /// current position in the hlist
    let mut p: pointer;
    // @!save_loc:integer; {\.{DVI} byte location upon entry}
    /// `DVI` byte location upon entry
    let save_loc: integer;
    // @!leader_box:pointer; {the leader box being replicated}
    // @!leader_wd:scaled; {width of leader box being replicated}
    // @!lx:scaled; {extra space between leader boxes}
    // @!outer_doing_leaders:boolean; {were we doing leaders?}
    // @!edge:scaled; {left edge of sub-box, or right edge of leader space}
    // @!glue_temp:real; {glue value before rounding}
    // @!cur_glue:real; {glue seen so far}
    /// glue seen so far
    let cur_glue: real;
    // @!cur_g:scaled; {rounded equivalent of |cur_glue| times the glue ratio}
    /// rounded equivalent of `cur_glue` times the glue ratio
    let cur_g: scaled;
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
    // save_loc:=dvi_offset+dvi_ptr; base_line:=cur_v; left_edge:=cur_h;
    save_loc = globals.dvi_offset + globals.dvi_ptr.get() as integer;
    base_line = globals.cur_v;
    left_edge = globals.cur_h;
    // while p<>null do @<Output node |p| for |hlist_out| and move to the next node,
    //   maintaining the condition |cur_v=base_line|@>;
    while p != null {
        Output_node_p_for_hlist_out_and_move_to_the_next_node__maintaining_the_condition_cur_v_eq_base_line!
            (globals, p, base_line, cur_g, g_sign);
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
    ok_nojump!()
}

use crate::pascal::real;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0135::glue_sign;
use crate::section_0150::glue_ord;
use crate::section_0586::push;
use crate::section_0601::dvi_pop;
use crate::section_0615::prune_movements;
