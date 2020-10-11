//! @ Here we have to remember to tell the |input_ln| routine not to
//! start with a |get|. If the file is empty, it is considered to
//! contain a single blank line.
//! @^system dependencies@>
//! @^empty line at end of file@>

// @<Read the first line...@>=
macro_rules! Read_the_first_line_of_the_new_file {
    ($globals:expr) => {
        // begin line:=1;
        // if input_ln(cur_file,false) then do_nothing;
        // firm_up_the_line;
        // if end_line_char_inactive then decr(limit)
        // else  buffer[limit]:=end_line_char;
        // first:=limit+1; loc:=start;
        // end
    }
}
