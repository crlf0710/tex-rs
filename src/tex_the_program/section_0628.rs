//! @ The `\\{synch}' operations here are intended to decrease the number of
//! bytes needed to specify horizontal and vertical motion in the \.{DVI} output.
//
// @<Output a leader box at |cur_h|, ...@>=
pub(crate) macro Output_a_leader_box_at_cur_h__then_advance_cur_h_by_leader_wd_plus_lx($globals:expr, $base_line:expr, $leader_box:expr, $leader_wd:expr, $lx:expr) {{
    /// what `dvi_h` and `dvi_v` should pop to
    let (save_h, save_v);
    /// were we doing leaders?
    let outer_doing_leaders;
    // begin cur_v:=base_line+shift_amount(leader_box); synch_v; save_v:=dvi_v;@/
    $globals.cur_v = $base_line + shift_amount!($globals, $leader_box);
    synch_v!($globals);
    save_v = $globals.dvi_v;
    // synch_h; save_h:=dvi_h; temp_ptr:=leader_box;
    synch_h!($globals);
    save_h = $globals.dvi_h;
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
    // dvi_v:=save_v; dvi_h:=save_h; cur_v:=base_line;
    $globals.dvi_v = save_v;
    $globals.dvi_h = save_h;
    $globals.cur_v = $base_line;
    // cur_h:=save_h+leader_wd+lx;
    $globals.cur_h = save_h + $leader_wd + $lx;
    // end
    use crate::section_0133::r#type;
    use crate::section_0135::shift_amount;
    use crate::section_0137::vlist_node;
    use crate::section_0616::synch_h;
    use crate::section_0616::synch_v;
    use crate::section_0619::hlist_out;
    use crate::section_0629::vlist_out;
}}
