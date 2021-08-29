//! ` `
// @<Subtract glue from |break...@>=
macro_rules! Subtract_glue_from_break_width {
    ($globals:expr, $s:expr) => {{
        /// points to a glue specification or a node ahead of `cur_p`
        let v: pointer;
        // begin v:=glue_ptr(s); break_width[1]:=break_width[1]-width(v);
        v = glue_ptr!($globals, $s);
        $globals.break_width[1] -= width!($globals, v);
        // break_width[2+stretch_order(v)]:=break_width[2+stretch_order(v)]-stretch(v);
        $globals.break_width[2 + stretch_order!($globals, v)] -= stretch!($globals, v);
        // break_width[6]:=break_width[6]-shrink(v);
        $globals.break_width[6] -= shrink!($globals, v);
        // end
    }}
}
