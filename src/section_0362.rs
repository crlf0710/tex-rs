//! ` `

// @<Read next line of file into |buffer|, or
//   |goto restart| if the file has ended@>=
macro_rules! Read_next_line_of_file_into_buffer_or_goto_restart_if_the_file_has_ended {
    ($globals:expr, $lbl_restart:lifetime) => {
        todo!();
        // begin incr(line); first:=start;
        // if not force_eof then
        //   begin if input_ln(cur_file,true) then {not end of file}
        //     firm_up_the_line {this sets |limit|}
        //   else force_eof:=true;
        //   end;
        // if force_eof then
        //   begin print_char(")"); decr(open_parens);
        //   update_terminal; {show user that file has been read}
        //   force_eof:=false;
        //   end_file_reading; {resume previous level}
        //   check_outer_validity; goto restart;
        //   end;
        // if end_line_char_inactive then decr(limit)
        // else  buffer[limit]:=end_line_char;
        // first:=limit+1; loc:=start; {ready to read}
        // end
    }
}