//! ` `
// @<Report an underfull vbox and |goto common_ending|, if...@>=
macro_rules! Report_an_underfull_vbox_and_goto_common_ending__if_this_box_is_sufficiently_bad {
    ($globals:expr, $x:expr) => {{
        // begin last_badness:=badness(x,total_stretch[normal]);
        $globals.last_badness = badness($globals, $x, $globals.total_stretch[glue_ord::normal]) as _;
        // if last_badness>vbadness then
        if $globals.last_badness > vbadness!($globals) {
            todo!("underfull");
            //   begin print_ln;
            //   if last_badness>100 then print_nl("Underfull")@+else print_nl("Loose");
            //   print(" \vbox (badness "); print_int(last_badness);
            // @.Underfull \\vbox...@>
            // @.Loose \\vbox...@>
            //   goto common_ending;
            //   end;
        }
        // end
        use crate::section_0108::badness;
    }}
}
