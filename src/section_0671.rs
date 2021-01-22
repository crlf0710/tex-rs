//! ` `
// @<Incorporate glue into the vertical totals@>=
macro_rules! Incorporate_glue_into_the_vertical_totals {
    ($globals:expr, $p:expr, $w:expr, $d:expr, $x:expr) => {{
        /// points to a glue specification
        let mut g: pointer;
        /// order of infinity
        let mut o: glue_ord;
        // begin x:=x+d; d:=0;@/
        $x = $x + $d;
        $d = scaled::zero();
        // g:=glue_ptr(p); x:=x+width(g);@/
        g = glue_ptr!($globals, $p);
        $x = $x + width!($globals, g);
        // o:=stretch_order(g); total_stretch[o]:=total_stretch[o]+stretch(g);
        o = stretch_order!($globals, g).into();
        $globals.total_stretch[o] += stretch!($globals, g);
        // o:=shrink_order(g); total_shrink[o]:=total_shrink[o]+shrink(g);
        o = shrink_order!($globals, g).into();
        $globals.total_shrink[o] += shrink!($globals, g);
        // if subtype(p)>=a_leaders then
        if subtype!($globals, $p) >= a_leaders {
            // begin g:=leader_ptr(p);
            g = leader_ptr!($globals, $p);
            // if width(g)>w then w:=width(g);
            if width!($globals, g) > $w {
                $w = width!($globals, g);
            }
            // end;
        }
        // end
        use crate::section_0149::a_leaders;
        use crate::section_0150::glue_ord;
    }}
}
