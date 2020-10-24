//! @ All of the easy branches of |get_next| have now been taken care of.
//! There is one more branch.
//
// @d end_line_char_inactive == (end_line_char<0)or(end_line_char>255)
#[cfg(not(feature = "unicode_support"))]
macro_rules! end_line_char_inactive {
    ($globals:expr) => {
        {
            end_line_char!($globals) < 0 || end_line_char!($globals) > 255
        }
    }
}

#[cfg(feature = "unicode_support")]
macro_rules! end_line_char_inactive {
    ($globals:expr) => {
        {
            // FIXME: needs to do something here.
            end_line_char!($globals) < 0 || end_line_char!($globals) > 255 && true
        }
    }
}

//
// @<Move to next line of file, or |goto restart|...@>=
macro_rules! Move_to_next_line_of_file_or_goto_restart_if_there_is_no_next_line_or_return_if_a_read_line_has_finished {
    ($globals:expr) => {
        // if name>17 then @<Read next line of file into |buffer|, or
        //   |goto restart| if the file has ended@>
        if name!($globals) > 17 {
            todo!();
        } else {
            // else  begin if not terminal_input then {\.{\\read} line has ended}
            if !terminal_input($globals) {
                /// `\read` line has ended
                const _ : () = ();
                // begin cur_cmd:=0; cur_chr:=0; return;
                $globals.cur_cmd = 0;
                $globals.cur_chr = cur_chr_type::new_zero();
                return_nojump!();
                // end;
            }
            //   if input_ptr>0 then {text was inserted during error recovery}
            //     begin end_file_reading; goto restart; {resume previous level}
            //     end;
            //   if selector<log_only then open_log_file;
            //   if interaction>nonstop_mode then
            if $globals.interaction > nonstop_mode {
                //     begin if end_line_char_inactive then incr(limit);
                // if limit=start then {previous line was empty}
                //   print_nl("(Please type a command or say `\end')");
                if limit!($globals) == start!($globals) {
                    print_nl($globals, strpool_str!("(Please type a command or say `\\end')"));
                }
                // @.Please type...@>
                // print_ln; first:=start;
                print_ln($globals);
                $globals.first = start!($globals).into();
                // prompt_input("*"); {input on-line into |buffer|}
                /// input on-line into `buffer`
                prompt_input!($globals, strpool_str!("*"));
                // @.*\relax@>
                //     limit:=last;
                //     if end_line_char_inactive then decr(limit)
                //     else  buffer[limit]:=end_line_char;
                //     first:=limit+1;
                //     loc:=start;
                //     end
            } else {
                //   else fatal_error("*** (job aborted, no legal \end found)");
                // @.job aborted@>
                //     {nonstop mode, which is intended for overnight batch processing,
                //     never waits for on-line input}
                //   end
            }
        }

        use crate::section_0304::terminal_input;
        use crate::section_0297::cur_chr_type;
        use crate::section_0073::nonstop_mode;
        use crate::section_0057::print_ln;
        use crate::section_0062::print_nl;
    }
}