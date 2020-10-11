// @ @<Input from external file, |goto restart| if no input found@>=
// @^inner loop@>
macro_rules! Input_from_external_file__goto_restart_if_no_input_found {
    ($globals:expr) => {
        region_backward_label! {
        'switch <-
        {
            // begin switch: if loc<=limit then {current line not yet finished}
            if loc!($globals) <= limit!($globals) {
                /// current line not yet finished
                const _ : () = ();
                // begin cur_chr:=buffer[loc]; incr(loc);
                // reswitch: cur_cmd:=cat_code(cur_chr);
                // @<Change state if necessary, and |goto switch| if the
                //   current character should be ignored,
                //   or |goto reswitch| if the current character
                //   changes to another@>;
                // end
            } else {
                // else  begin state:=new_line;@/
                state!($globals) = new_line;
                // @<Move to next line of file,
                //   or |goto restart| if there is no next line,
                //   or |return| if a \.{\\read} line has finished@>;
                // check_interrupt;
                // goto switch;
                goto_backward_label!('switch);
                // end;
            }
        }
        |'switch|
        }
        // end
        use crate::section_0303::new_line;
    }
}
