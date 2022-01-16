//! ` `

// @<Report a tight vbox and |goto common_ending|, if...@>=
pub(crate) macro Report_a_tight_vbox_and_goto_common_ending__if_this_box_is_sufficiently_bad($globals:expr, $x:expr) {{
    // begin last_badness:=badness(-x,total_shrink[normal]);
    $globals.last_badness = badness($globals, -$x, $globals.total_shrink[glue_ord::normal]) as _;
    // if last_badness>vbadness then
    if $globals.last_badness > vbadness!($globals) {
        // begin print_ln; print_nl("Tight \vbox (badness "); print_int(last_badness);
        // @.Tight \\vbox...@>
        // goto common_ending;
        // end;
        todo!("Report a tight vbox");
    }
    // end
    use crate::section_0108::badness;
    use crate::section_0150::glue_ord;
    use crate::section_0236::vbadness;
}}
