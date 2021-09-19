// @ @<Input from external file, |goto restart| if no input found@>=
// @^inner loop@>
pub(crate) macro Input_from_external_file__goto_restart_if_no_input_found
    ($globals:expr, $lbl_restart:lifetime) {
        crate::trace_span_verbose!("Input from external...");
        crate::region_backward_label! {
        'switch <-
        {
            crate::trace_expr_verbose!("loc, limit = ({:?}, {:?})", loc!($globals), limit!($globals));
            // begin switch: if loc<=limit then {current line not yet finished}
            if loc!($globals) <= limit!($globals) {
                /// current line not yet finished
                const _ : () = ();
                // begin cur_chr:=buffer[loc]; incr(loc);
                $globals.cur_chr = $globals.buffer[loc!($globals)].into();
                crate::trace_expr_verbose!("cur_chr = {:?}", $globals.cur_chr);
                incr!(loc!($globals));
                // reswitch: cur_cmd:=cat_code(cur_chr);
                crate::region_backward_label! {
                'reswitch <-
                {
                    $globals.cur_cmd = cat_code!($globals, $globals.cur_chr.into()) as _;
                    crate::trace_expr_verbose!("cur_cmd = {:?}", $globals.cur_cmd);
                    // @<Change state if necessary, and |goto switch| if the
                    //   current character should be ignored,
                    //   or |goto reswitch| if the current character
                    //   changes to another@>;
                    crate::section_0344::Change_state_if_necessary_and_goto_switch_if_the_current_character_should_be_ignored_or_goto_reswitch_if_the_current_character_changes_to_another!
                        ($globals, 'switch, 'reswitch);
                    // end
                }
                |'reswitch|
                }
            } else {
                // else  begin state:=new_line;@/
                state!($globals) = new_line;
                // @<Move to next line of file,
                //   or |goto restart| if there is no next line,
                //   or |return| if a \.{\\read} line has finished@>;
                crate::section_0360::Move_to_next_line_of_file_or_goto_restart_if_there_is_no_next_line_or_return_if_a_read_line_has_finished!
                    ($globals, $lbl_restart);
                // check_interrupt;
                check_interrupt!($globals);
                // goto switch;
                crate::goto_backward_label!('switch);
                // end;
            }
        }
        |'switch|
        }
        // end
        use crate::section_0016::incr;
        use crate::section_0096::check_interrupt;
        use crate::section_0302::state;
        use crate::section_0036::loc;
        use crate::section_0230::cat_code;
        use crate::section_0302::limit;
        use crate::section_0303::new_line;
    }
