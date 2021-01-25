//! ` `
// @<Incorporate glue into the horizontal totals@>=
macro_rules! Incorporate_glue_into_the_horizontal_totals {
    ($globals:expr, $p:expr, $h:expr, $d:expr, $x:expr) => {{
        /// points to a glue specification
        let mut g: pointer;
        /// order of infinity
        let mut o: glue_ord;

        // begin g:=glue_ptr(p); x:=x+width(g);@/
        g = glue_ptr!($globals, $p);
        $x = $x + width!($globals, g);
        // o:=stretch_order(g); total_stretch[o]:=total_stretch[o]+stretch(g);
        o = stretch_order!($globals, g).into();
        $globals.total_stretch[o] += stretch!($globals, g);
        // o:=shrink_order(g); total_shrink[o]:=total_shrink[o]+shrink(g);
        o = shrink_order!($globals, g).into();
        $globals.total_shrink[o] += shrink!($globals, g);
        // if subtype(p)>=a_leaders then
        if subtype!($globals, $p) as integer >= glue_node_subtype::a_leaders as integer {
            // begin g:=leader_ptr(p);
            g = leader_ptr!($globals, $p);
            // if height(g)>h then h:=height(g);
            if height!($globals, g) > $h {
                $h = height!($globals, g);
            }
            // if depth(g)>d then d:=depth(g);
            if depth!($globals, g) > $d {
                $d = depth!($globals, g);
            }
            // end;
        }
        // end
        use crate::pascal::integer;
        use crate::section_0149::glue_node_subtype;
        use crate::section_0150::glue_ord;
    }};
}
