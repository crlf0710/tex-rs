//! ` `

// @<Clear dimensions to zero@>=
macro_rules! Clear_dimensions_to_zero {
    ($globals:expr, $d:expr, $x:expr) => {{
        // d:=0; x:=0;
        $d = scaled::zero();
        $x = scaled::zero();
        // total_stretch[normal]:=0; total_shrink[normal]:=0;
        $globals.total_stretch[normal] = scaled::zero();
        $globals.total_shrink[normal] = scaled::zero();
        // total_stretch[fil]:=0; total_shrink[fil]:=0;
        $globals.total_stretch[fil] = scaled::zero();
        $globals.total_shrink[fil] = scaled::zero();
        // total_stretch[fill]:=0; total_shrink[fill]:=0;
        $globals.total_stretch[fill] = scaled::zero();
        $globals.total_shrink[fill] = scaled::zero();
        // total_stretch[filll]:=0; total_shrink[filll]:=0
        $globals.total_stretch[filll] = scaled::zero();
        $globals.total_shrink[filll] = scaled::zero();

        use crate::section_0150::glue_ord::*;
    }}
}
