//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0231 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // assign_toks: if chr_code>=toks_base then
        if $cmd == assign_toks {
            if $chr_code.get() >= toks_base {
                // begin print_esc("toks"); print_int(chr_code-toks_base);
                print_esc($globals, strpool_str!("toks"));
                print_int($globals, $chr_code.get() as integer - toks_base as integer);
                // end
            }
            // else  case chr_code of
            else {
                todo!("assign_toks other");
                // output_routine_loc: print_esc("output");
                // every_par_loc: print_esc("everypar");
                // every_math_loc: print_esc("everymath");
                // every_display_loc: print_esc("everydisplay");
                // every_hbox_loc: print_esc("everyhbox");
                // every_vbox_loc: print_esc("everyvbox");
                // every_job_loc: print_esc("everyjob");
                // every_cr_loc: print_esc("everycr");
                // othercases print_esc("errhelp")
                // endcases;
            }
            use crate::section_0230::toks_base;
            true
        } else {
            false
        }
    }}
}

