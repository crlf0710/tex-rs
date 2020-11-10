//! ` `

// @<Read next line of file into |buffer|, or
//   |goto restart| if the file has ended@>=
macro_rules! Read_next_line_of_file_into_buffer_or_goto_restart_if_the_file_has_ended {
    ($globals:expr, $lbl_restart:lifetime) => {{
        // begin incr(line); first:=start;
        incr!($globals.line);
        $globals.first = start!($globals).into();
        // if not force_eof then
        if !$globals.force_eof {
            // begin if input_ln(cur_file,true) then {not end of file}
            /// not end of file
            if input_ln(make_globals_io_view!($globals), &mut cur_file!($globals), true) {
                // firm_up_the_line {this sets |limit|}
                /// this sets `limit`
                firm_up_the_line($globals);
            }
            // else force_eof:=true;
            else {
                $globals.force_eof = true;
            }
            // end;
        }
        // if force_eof then
        if $globals.force_eof {
            // begin print_char(")"); decr(open_parens);
            print_char(make_globals_io_string_view!($globals), ASCII_code_literal!(b')'));
            decr!($globals.open_parens);
            trace_expr!("open_parens = {:?}", $globals.open_parens);
            // update_terminal; {show user that file has been read}
            /// show user that file has been read
            update_terminal($globals);
            // force_eof:=false;
            $globals.force_eof = false;
            // end_file_reading; {resume previous level}
            /// resume previous level
            end_file_reading($globals);
            // check_outer_validity; goto restart;
            check_outer_validity($globals);
            goto_backward_label!($lbl_restart);
            // end;
        }
        // if end_line_char_inactive then decr(limit)
        if end_line_char_inactive!($globals) {
            decr!(limit!($globals));
        }
        // else  buffer[limit]:=end_line_char;
        else {
            $globals.buffer[limit!($globals)] = end_line_char!($globals).into();
        }
        // first:=limit+1; loc:=start; {ready to read}
        $globals.first = (limit!($globals) + 1).into();
        loc!($globals) = start!($globals);
        /// ready to read
        const _ : () = ();
        // end
        use crate::section_0004::TeXGlobalsIoView;
        use crate::section_0004::TeXGlobalsIoStringView;
        use crate::section_0031::input_ln;
        use crate::section_0034::update_terminal;
        use crate::section_0058::print_char;
        use crate::section_0329::end_file_reading;
        use crate::section_0336::check_outer_validity;
        use crate::section_0363::firm_up_the_line;
    }}
}