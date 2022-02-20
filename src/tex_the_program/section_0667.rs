//! ` `

// @<Report a tight hbox and |goto common_ending|, if...@>=
pub(crate) macro Report_a_tight_hbox_and_goto_common_ending__if_this_box_is_sufficiently_bad($globals:expr, $x:expr, $lbl_common_ending:lifetime) {{
    // begin last_badness:=badness(-x,total_shrink[normal]);
    $globals.last_badness = badness($globals, -$x, $globals.total_shrink[glue_ord::normal]) as _;
    // if last_badness>hbadness then
    if $globals.last_badness > hbadness!($globals) {
        // begin print_ln; print_nl("Tight \hbox (badness "); print_int(last_badness);
        print_ln(make_globals_io_string_log_view!($globals));
        print_nl($globals, crate::strpool_str!("Tight \\hbox (badness "));
        print_int($globals, $globals.last_badness);
        // @.Tight \\hbox...@>
        // goto common_ending;
        crate::goto_forward_label!($lbl_common_ending);
        // end;
    }
    // end
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0057::print_ln;
    use crate::section_0062::print_nl;
    use crate::section_0065::print_int;
    use crate::section_0108::badness;
    use crate::section_0150::glue_ord;
    use crate::section_0236::hbadness;
}}
