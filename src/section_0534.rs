//! @ The |open_log_file| routine is used to open the transcript file and to help
//! it catch up to what has previously been printed on the terminal.
//
// @p procedure open_log_file;
#[allow(unused_variables)]
pub(crate) fn open_log_file(globals: &mut TeXGlobals) {
    todo!();
    // var old_setting:0..max_selector; {previous |selector| setting}
    // @!k:0..buf_size; {index into |months| and |buffer|}
    // @!l:0..buf_size; {end of first input line}
    // @!months:packed array [1..36] of char; {abbreviations of month names}
    // begin old_setting:=selector;
    // if job_name=0 then job_name:="texput";
    // @.texput@>
    // pack_job_name(".log");
    // while not a_open_out(log_file) do @<Try to get a different log file name@>;
    // log_name:=a_make_name_string(log_file);
    // selector:=log_only; log_opened:=true;
    // @<Print the banner line, including the date and time@>;
    // input_stack[input_ptr]:=cur_input; {make sure bottom level is in memory}
    // print_nl("**");
    // @.**@>
    // l:=input_stack[0].limit_field; {last position of first line}
    // if buffer[l]=end_line_char then decr(l);
    // for k:=1 to l do print(buffer[k]);
    // print_ln; {now the transcript file contains the first line of input}
    // selector:=old_setting+2; {|log_only| or |term_and_log|}
    // end;
}

use crate::section_0004::TeXGlobals;
