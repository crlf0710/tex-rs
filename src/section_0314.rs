//! ` `

// @<Print type of token list@>=
macro_rules! Print_type_of_token_list {
    ($globals:expr) => {{
        let token_type = token_type!($globals);
        // case token_type of
        // parameter: print_nl("<argument> ");
        if token_type == parameter {
            print_nl($globals, strpool_str!("<argument> "));
        }
        // u_template,v_template: print_nl("<template> ");
        // backed_up: if loc=null then print_nl("<recently read> ")
        else if token_type == backed_up {
            if loc!($globals) == null {
                print_nl($globals, strpool_str!("<recently read> "));
            }
            // else print_nl("<to be read again> ");
            else {
                print_nl($globals, strpool_str!("<to be read again> "));
            }
        }
        // inserted: print_nl("<inserted text> ");
        else if token_type == inserted {
            print_nl($globals, strpool_str!("<inserted text> "));
        }
        // macro: begin print_ln; print_cs(name);
        //   end;
        // output_text: print_nl("<output> ");
        // every_par_text: print_nl("<everypar> ");
        // every_math_text: print_nl("<everymath> ");
        // every_display_text: print_nl("<everydisplay> ");
        // every_hbox_text: print_nl("<everyhbox> ");
        // every_vbox_text: print_nl("<everyvbox> ");
        // every_job_text: print_nl("<everyjob> ");
        // every_cr_text: print_nl("<everycr> ");
        // mark_text: print_nl("<mark> ");
        // write_text: print_nl("<write> ");
        // othercases print_nl("?") {this should never happen}
        else {
            trace_error_expr!("token_type = {}", token_type);
            /// this should never happen
            print_nl($globals, strpool_str!("?"));
        }
        // endcases
        use crate::section_0307::*;
    }}
}
