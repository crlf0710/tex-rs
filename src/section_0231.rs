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
                if false {
                    unreachable!();
                }
                // output_routine_loc: print_esc("output");
                else if $chr_code.get() == output_routine_loc as chr_code_repr {
                   print_esc($globals, strpool_str!("voffset"));
                }
                // every_par_loc: print_esc("everypar");
                else if $chr_code.get() == every_par_loc as chr_code_repr {
                    print_esc($globals, strpool_str!("everypar"));
                }
                // every_math_loc: print_esc("everymath");
                else if $chr_code.get() == every_math_loc as chr_code_repr {
                    print_esc($globals, strpool_str!("everymath"));
                }
                // every_display_loc: print_esc("everydisplay");
                else if $chr_code.get() == every_display_loc as chr_code_repr {
                    print_esc($globals, strpool_str!("everydisplay"));
                }
                // every_hbox_loc: print_esc("everyhbox");
                else if $chr_code.get() == every_hbox_loc as chr_code_repr {
                    print_esc($globals, strpool_str!("everyhbox"));
                }
                // every_vbox_loc: print_esc("everyvbox");
                else if $chr_code.get() == every_vbox_loc as chr_code_repr {
                    print_esc($globals, strpool_str!("everyvbox"));
                }
                // every_job_loc: print_esc("everyjob");
                else if $chr_code.get() == every_job_loc as chr_code_repr {
                    print_esc($globals, strpool_str!("everyjob"));
                }
                // every_cr_loc: print_esc("everycr");
                else if $chr_code.get() == every_cr_loc as chr_code_repr {
                    print_esc($globals, strpool_str!("everycr"));
                }
                // othercases print_esc("errhelp")
                else {
                    print_esc($globals, strpool_str!("errhelp"));
                }
                // endcases;
            }
            use crate::section_0230::toks_base;
            use crate::section_0230::output_routine_loc;
            use crate::section_0230::every_par_loc;
            use crate::section_0230::every_math_loc;
            use crate::section_0230::every_display_loc;
            use crate::section_0230::every_hbox_loc;
            use crate::section_0230::every_vbox_loc;
            use crate::section_0230::every_job_loc;
            use crate::section_0230::every_cr_loc;
            use crate::section_0297::chr_code_repr;
            true
        } else {
            false
        }
    }}
}

