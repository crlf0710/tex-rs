//! @ All of the easy branches of |get_next| have now been taken care of.
//! There is one more branch.
//
// @d end_line_char_inactive == (end_line_char<0)or(end_line_char>255)
#[cfg(not(feature = "unicode_support"))]
pub(crate) macro end_line_char_inactive($globals:expr) {{
    crate::section_0236::end_line_char!($globals) < 0
        || crate::section_0236::end_line_char!($globals) > 255
}}

#[cfg(feature = "unicode_support")]
pub(crate) macro end_line_char_inactive($globals:expr) {{
    // FIXME: needs to do something here.
    crate::section_0236::end_line_char!($globals) < 0
        || crate::section_0236::end_line_char!($globals) > 255 && true
}}

//
// @<Move to next line of file, or |goto restart|...@>=
pub(crate) macro Move_to_next_line_of_file_or_goto_restart_if_there_is_no_next_line_or_return_if_a_read_line_has_finished($globals:expr, $lbl_restart:lifetime) {
    // if name>17 then @<Read next line of file into |buffer|, or
    //   |goto restart| if the file has ended@>
    crate::trace_expr_verbose!("name = {}", name!($globals));
    if name!($globals) > 17 {
        crate::section_0362::Read_next_line_of_file_into_buffer_or_goto_restart_if_the_file_has_ended!(
            $globals,
            $lbl_restart
        );
    } else {
        // else  begin if not terminal_input then {\.{\\read} line has ended}
        if !terminal_input($globals) {
            /// `\read` line has ended
            const _: () = ();
            // begin cur_cmd:=0; cur_chr:=0; return;
            $globals.cur_cmd = 0;
            $globals.cur_chr = chr_code_type::zero();
            crate::return_nojump!();
            // end;
        }
        // if input_ptr>0 then {text was inserted during error recovery}
        if $globals.input_ptr > 0 {
            /// text was inserted during error recovery
            const _: () = ();
            // begin end_file_reading; goto restart; {resume previous level}
            end_file_reading($globals);
            crate::goto_backward_label!($lbl_restart);
            // end;
        }
        // if selector<log_only then open_log_file;
        if $globals.selector < log_only {
            open_log_file($globals);
        }
        // if interaction>nonstop_mode then
        if $globals.interaction > nonstop_mode {
            // begin if end_line_char_inactive then incr(limit);
            if end_line_char_inactive!($globals) {
                incr!(limit!($globals));
            }
            // if limit=start then {previous line was empty}
            //   print_nl("(Please type a command or say `\end')");
            if limit!($globals) == start!($globals) {
                print_nl(
                    $globals,
                    crate::strpool_str!("(Please type a command or say `\\end')"),
                );
            }
            // @.Please type...@>
            // print_ln; first:=start;
            print_ln(make_globals_io_string_log_view!($globals));
            $globals.first = start!($globals).into();
            // prompt_input("*"); {input on-line into |buffer|}
            /// input on-line into `buffer`
            prompt_input!($globals, crate::strpool_str!("*"));
            // @.*\relax@>
            // limit:=last;
            limit!($globals) = $globals.last.get();
            // if end_line_char_inactive then decr(limit)
            if end_line_char_inactive!($globals) {
                decr!(limit!($globals));
            } else {
                // else  buffer[limit]:=end_line_char;
                $globals.buffer[limit!($globals)] = end_line_char!($globals).into();
            }
            // first:=limit+1;
            $globals.first = (limit!($globals) + 1).into();
            // loc:=start;
            loc!($globals) = start!($globals);
            // end
        } else {
            todo!();
            //   else fatal_error("*** (job aborted, no legal \end found)");
            // @.job aborted@>
            //     {nonstop mode, which is intended for overnight batch processing,
            //     never waits for on-line input}
            //   end
        }
    }

    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0016::incr;
    use crate::section_0016::decr;
    use crate::section_0036::loc;
    use crate::section_0054::log_only;
    use crate::section_0057::print_ln;
    use crate::section_0062::print_nl;
    use crate::section_0071::prompt_input;
    use crate::section_0073::nonstop_mode;
    use crate::section_0236::end_line_char;
    use crate::section_0297::chr_code_type;
    use crate::section_0302::limit;
    use crate::section_0302::name;
    use crate::section_0302::start;
    use crate::section_0304::terminal_input;
    use crate::section_0329::end_file_reading;
    use crate::section_0534::open_log_file;
}
