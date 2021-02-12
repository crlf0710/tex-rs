//! ` `
// @<Determine the stretch order@>=
macro_rules! Determine_the_stretch_order {
    ($globals:expr, $o:expr) => {{
        // if total_stretch[filll]<>0 then o:=filll
        if $globals.total_stretch[glue_ord::filll] != scaled::zero() {
            $o = glue_ord::filll;
        }
        // else if total_stretch[fill]<>0 then o:=fill
        else if $globals.total_stretch[glue_ord::fill] != scaled::zero() {
            $o = glue_ord::fill;
        }
        // else if total_stretch[fil]<>0 then o:=fil
        else if $globals.total_stretch[glue_ord::fil] != scaled::zero() {
            $o = glue_ord::fil;
        }
        // else o:=normal
        else {
            $o = glue_ord::normal;
        }
    }}
}
