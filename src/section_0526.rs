//! @ Now let's consider the ``driver''
//! routines by which \TeX\ deals with file names
//! in a system-independent manner.  First comes a procedure that looks for a
//! file name in the input by calling |get_x_token| for the information.
//
// @p procedure scan_file_name;
pub(crate) fn scan_file_name() {
    // label done;
    // begin name_in_progress:=true; begin_name;
    // @<Get the next non-blank non-call...@>;
    // loop@+begin if (cur_cmd>other_char)or(cur_chr>255) then {not a character}
    //     begin back_input; goto done;
    //     end;
    //   if not more_name(cur_chr) then goto done;
    //   get_x_token;
    //   end;
    // done: end_name; name_in_progress:=false;
    // end;
}
