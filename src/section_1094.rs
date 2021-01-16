//! @ A paragraph ends when a |par_end| command is sensed, or when we are in
//! horizontal mode when reaching the right brace of vertical-mode routines
//! like \.{\\vbox}, \.{\\insert}, or \.{\\output}.
//
// @<Cases of |main_control| that build...@>=
macro_rules! Cases_of_main_control_that_build_boxes_and_lists_1094 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        trace_span!("Cases of `main_control` that build...1094");
        // vmode+par_end: begin normal_paragraph;
        if $abs_mode_plus_cur_cmd == vmode as u16 + par_end as u16 {
            normal_paragraph($globals)?;
            // if mode>0 then build_page;
            if mode!($globals) > 0 {
                build_page($globals);
            }
            // end;
            use crate::section_1070::normal_paragraph;
            use crate::section_0994::build_page;
            true
        }
        // hmode+par_end: begin if align_state<0 then off_save; {this tries to
        //     recover from an alignment that didn't end properly}
        else if $abs_mode_plus_cur_cmd == hmode as u16 + par_end as u16 {
            if $globals.align_state < 0 {
                /// this tries to recover from an alignment that didn't end properly
                const _ : () = ();
                off_save($globals);
            }
            // end_graf; {this takes us to the enclosing mode, if |mode>0|}
            /// this takes us to the enclosing mode, if |mode>0|
            const _ : () = ();
            end_graf($globals);
            // if mode=vmode then build_page;
            if mode!($globals).get() == vmode {
                build_page($globals);
            }
            // end;
            use crate::section_0994::build_page;
            use crate::section_1064::off_save;
            use crate::section_1096::end_graf;
            true
        }
        // hmode+stop,hmode+vskip,hmode+hrule,hmode+un_vbox,hmode+halign: head_for_vmode;
        else if $abs_mode_plus_cur_cmd == hmode as u16 + stop as u16 ||
            $abs_mode_plus_cur_cmd == hmode as u16 + vskip as u16 ||
            $abs_mode_plus_cur_cmd == hmode as u16 + hrule as u16 ||
            $abs_mode_plus_cur_cmd == hmode as u16 + un_vbox as u16 ||
            $abs_mode_plus_cur_cmd == hmode as u16 + halign as u16 {
            head_for_vmode($globals);
            use crate::section_1095::head_for_vmode;
            true
        } else {
            false
        }
    }}
}