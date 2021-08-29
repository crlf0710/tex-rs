//! ` `
// @<Report an underfull vbox and |goto common_ending|, if...@>=
macro_rules! Report_an_underfull_vbox_and_goto_common_ending__if_this_box_is_sufficiently_bad {
    ($globals:expr, $x:expr, $lbl_common_ending:lifetime) => {{
        // begin last_badness:=badness(x,total_stretch[normal]);
        $globals.last_badness = badness($globals, $x, $globals.total_stretch[glue_ord::normal]) as _;
        // if last_badness>vbadness then
        if $globals.last_badness > vbadness!($globals) {
            // begin print_ln;
            print_ln(make_globals_io_string_log_view!($globals));
            // if last_badness>100 then print_nl("Underfull")@+else print_nl("Loose");
            if $globals.last_badness > 100 {
                print_nl($globals, strpool_str!("Underfull"));
            } else {
                print_nl($globals, strpool_str!("Loose"));
            }
            // print(" \vbox (badness "); print_int(last_badness);
            print($globals, strpool_str!(" \\vbox (badness ").get() as _);
            print_int($globals, $globals.last_badness);
            // @.Underfull \\vbox...@>
            // @.Loose \\vbox...@>
            // goto common_ending;
            goto_forward_label!($lbl_common_ending);
            // end;
        }
        // end
        use crate::section_0004::TeXGlobalsIoStringLogView;
        use crate::section_0057::print_ln;
        use crate::section_0059::print;
        use crate::section_0062::print_nl;
        use crate::section_0065::print_int;
        use crate::section_0108::badness;
    }}
}
