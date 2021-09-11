//! @ The |synch_v| here allows the \.{DVI} output to use one-byte commands
//! for adjusting |v| in most cases, since the baselineskip distance will
//! usually be constant.
//
// @<Output a box in a vlist@>=
pub(crate) macro Output_a_box_in_a_vlist($globals:expr, $p:expr, $left_edge:expr) {{
    // if list_ptr(p)=null then cur_v:=cur_v+height(p)+depth(p)
    if list_ptr!($globals, $p) == null {
        $globals.cur_v += height!($globals, $p) + depth!($globals, $p);
    }
    // else  begin cur_v:=cur_v+height(p); synch_v;
    else {
        /// what `dvi_h` and `dvi_v` should pop to
        let (save_h, save_v): (scaled, scaled);
        $globals.cur_v += height!($globals, $p);
        synch_v!($globals);
        // save_h:=dvi_h; save_v:=dvi_v;
        save_h = $globals.dvi_h;
        save_v = $globals.dvi_v;
        // cur_h:=left_edge+shift_amount(p); {shift the box right}
        /// shift the box right
        const _: () = ();
        $globals.cur_h = $left_edge + shift_amount!($globals, $p);
        // temp_ptr:=p;
        $globals.temp_ptr = $p;
        // if type(p)=vlist_node then vlist_out@+else hlist_out;
        if r#type!($globals, $p) == vlist_node {
            vlist_out($globals)?;
        } else {
            hlist_out($globals)?;
        }
        // dvi_h:=save_h; dvi_v:=save_v;
        $globals.dvi_h = save_h;
        $globals.dvi_v = save_v;
        // cur_v:=save_v+depth(p); cur_h:=left_edge;
        $globals.cur_v = save_v + depth!($globals, $p);
        $globals.cur_h = $left_edge;
        // end
    }
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::list_ptr;
    use crate::section_0135::shift_amount;
    use crate::section_0137::vlist_node;
    use crate::section_0616::synch_v;
    use crate::section_0619::hlist_out;
    use crate::section_0629::vlist_out;
}}
