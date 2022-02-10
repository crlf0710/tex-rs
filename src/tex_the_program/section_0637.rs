//! @ When we reach this part of the program, |cur_v| indicates the top of a
//! leader box, not its baseline.
//
// @<Output a leader box at |cur_v|, ...@>=
pub(crate) macro Output_a_leader_box_at_cur_v__then_advance_cur_v_by_leader_ht_plus_lx($globals:expr, $left_edge:expr, $leader_box:expr, $lx:expr, $leader_ht:expr) {{
    /// what `dvi_h` and `dvi_v` should pop to
    let (save_h, save_v): (scaled, scaled);
    /// were we doing leaders?
    let outer_doing_leaders;
    // begin cur_h:=left_edge+shift_amount(leader_box); synch_h; save_h:=dvi_h;@/
    $globals.cur_h = $left_edge + shift_amount!($globals, $leader_box);
    synch_h!($globals);
    save_h = $globals.dvi_h;
    // cur_v:=cur_v+height(leader_box); synch_v; save_v:=dvi_v;
    $globals.cur_v = $globals.cur_v + height!($globals, $leader_box);
    synch_v!($globals);
    save_v = $globals.dvi_v;
    // temp_ptr:=leader_box;
    $globals.temp_ptr = $leader_box;
    // outer_doing_leaders:=doing_leaders; doing_leaders:=true;
    outer_doing_leaders = $globals.doing_leaders;
    $globals.doing_leaders = true;
    // if type(leader_box)=vlist_node then vlist_out@+else hlist_out;
    if r#type!($globals, $leader_box) == vlist_node {
        vlist_out($globals)?;
    } else {
        hlist_out($globals)?;
    }
    // doing_leaders:=outer_doing_leaders;
    $globals.doing_leaders = outer_doing_leaders;
    // dvi_v:=save_v; dvi_h:=save_h; cur_h:=left_edge;
    $globals.dvi_v = save_v;
    $globals.dvi_h = save_h;
    $globals.cur_h = $left_edge;
    // cur_v:=save_v-height(leader_box)+leader_ht+lx;
    $globals.cur_v = save_v - height!($globals, $leader_box) + $leader_ht + $lx;
    // end
    use crate::section_0101::scaled;
    use crate::section_0133::r#type;
    use crate::section_0135::height;
    use crate::section_0135::shift_amount;
    use crate::section_0137::vlist_node;
    use crate::section_0616::synch_h;
    use crate::section_0616::synch_v;
    use crate::section_0619::hlist_out;
    use crate::section_0629::vlist_out;
}}
