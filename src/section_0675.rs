//! ` `
// @<Finish issuing a diagnostic message for an overfull or underfull vbox@>=
macro_rules! Finish_issuing_a_diagnostic_message_for_an_overfull_or_underfull_vbox {
    ($globals:expr, $r:expr) => {{
        // if output_active then print(") has occurred while \output is active")
        if $globals.output_active {
            print($globals, strpool_str!(") has occurred while \\output is active").get() as _);
        }
        // else  begin if pack_begin_line<>0 then {it's actually negative}
        else {
            todo!("xxx");
            //   begin print(") in alignment at lines ");
            //   print_int(abs(pack_begin_line));
            //   print("--");
            //   end
            // else print(") detected at line ");
            // print_int(line);
            // print_ln;@/
            // end;
        }
        // begin_diagnostic; show_box(r); end_diagnostic(true)
        begin_diagnostic($globals);
        show_box($globals, $r);
        end_diagnostic($globals, true);
        use crate::section_0059::print;
        use crate::section_0198::show_box;
        use crate::section_0245::begin_diagnostic;
        use crate::section_0245::end_diagnostic;
    }}
}
