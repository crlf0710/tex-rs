//! ` `
// @<Determine the shrink order@>=
macro_rules! Determine_the_shrink_order {
    ($globals:expr, $o:expr) => {{
        // if total_shrink[filll]<>0 then o:=filll
        if $globals.total_shrink[glue_ord::filll] != scaled::zero() {
            $o = glue_ord::filll;
        }
        // else if total_shrink[fill]<>0 then o:=fill
        else if $globals.total_shrink[glue_ord::fill] != scaled::zero() {
            $o = glue_ord::fill;
        }
        // else if total_shrink[fil]<>0 then o:=fil
        else if $globals.total_shrink[glue_ord::fil] != scaled::zero() {
            $o = glue_ord::fil;
        }
        // else o:=normal
        else {
            $o = glue_ord::normal;
        }
    }}
}