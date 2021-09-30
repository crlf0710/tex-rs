//! ` `
// @<Output a box in an hlist@>=
pub(crate) macro Output_a_box_in_an_hlist($globals:expr, $p:expr, $base_line:expr) {{
    // if list_ptr(p)=null then cur_h:=cur_h+width(p)
    if list_ptr!($globals, $p) == null {
        $globals.cur_h += width!($globals, $p);
    }
    // else  begin save_h:=dvi_h; save_v:=dvi_v;
    else {
        /// what `dvi_h` and `dvi_v` should pop to
        let (save_h, save_v): (scaled, scaled);
        /// left edge of sub-box, or right edge of leader space
        let edge: scaled;
        save_h = $globals.dvi_h;
        save_v = $globals.dvi_v;
        // cur_v:=base_line+shift_amount(p); {shift the box down}
        /// shift the box down
        const _: () = ();
        $globals.cur_v = $base_line + shift_amount!($globals, $p);
        // temp_ptr:=p; edge:=cur_h;
        $globals.temp_ptr = $p;
        edge = $globals.cur_h;
        // if type(p)=vlist_node then vlist_out@+else hlist_out;
        if r#type!($globals, $p) == vlist_node {
            vlist_out($globals)?;
        } else {
            hlist_out($globals)?;
        }
        // dvi_h:=save_h; dvi_v:=save_v;
        $globals.dvi_h = save_h;
        $globals.dvi_v = save_v;
        // cur_h:=edge+width(p); cur_v:=base_line;
        $globals.cur_h = edge + width!($globals, $p);
        $globals.cur_v = $base_line;
        // end
    }
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0133::r#type;
    use crate::section_0135::list_ptr;
    use crate::section_0135::shift_amount;
    use crate::section_0135::width;
    use crate::section_0137::vlist_node;
    use crate::section_0619::hlist_out;
    use crate::section_0629::vlist_out;
}}
