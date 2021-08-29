//! @ The |open_log_file| routine is used to open the transcript file and to help
//! it catch up to what has previously been printed on the terminal.
//
// @p procedure open_log_file;
pub(crate) fn open_log_file(globals: &mut TeXGlobals) {
    // var old_setting:0..max_selector; {previous |selector| setting}
    /// previous `selector` setting
    let old_setting;
    // @!k:0..buf_size; {index into |months| and |buffer|}
    // @!l:0..buf_size; {end of first input line}
    /// end of first input line
    let mut l;
    // @!months:packed array [1..36] of char; {abbreviations of month names}
    // begin old_setting:=selector;
    old_setting = globals.selector;
    // if job_name=0 then job_name:="texput";
    if globals.job_name == 0 {
        globals.job_name = strpool_str!("texput");
    }
    // @.texput@>
    // pack_job_name(".log");
    pack_job_name(globals, strpool_str!(".log"));
    // while not a_open_out(log_file) do @<Try to get a different log file name@>;
    while !a_open_out(make_globals_filename_view!(globals), &mut globals.log_file) {
        todo!("");
    }
    // log_name:=a_make_name_string(log_file);
    globals.log_name = a_make_name_string(make_globals_io_string_view!(globals), &mut globals.log_file);
    // selector:=log_only; log_opened:=true;
    globals.selector = log_only.into();
    globals.log_opened = true;
    // @<Print the banner line, including the date and time@>;
    Print_the_banner_line__including_the_date_and_time!(globals);
    // input_stack[input_ptr]:=cur_input; {make sure bottom level is in memory}
    /// make sure bottom level is in memory
    const _ : () = ();
    globals.input_stack[globals.input_ptr] = globals.cur_input;
    // print_nl("**");
    print_nl(globals, strpool_str!("**"));
    // @.**@>
    // l:=input_stack[0].limit_field; {last position of first line}
    /// last position of first line
    const _ : () = ();
    l = globals.input_stack[0].limit_field;
    // if buffer[l]=end_line_char then decr(l);
    if globals.buffer[l].numeric_value() == end_line_char!(globals) as ASCII_code_repr {
        decr!(l);
    }
    // for k:=1 to l do print(buffer[k]);
    for k in 1..=l {
        let ch = globals.buffer[k];
        if ch.numeric_value() > 255 {
            panic!("unsupported yet");
        }
        print(globals, ch.numeric_value() as _);
    }
    // print_ln; {now the transcript file contains the first line of input}
    /// now the transcript file contains the first line of input
    print_ln(make_globals_io_string_log_view!(globals));
    // selector:=old_setting+2; {|log_only| or |term_and_log|}
    /// `log_only` or `term_and_log`
    const _ : () = ();
    globals.selector = old_setting + 2;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsFilenameView;
use crate::section_0004::TeXGlobalsIoStringView;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0018::ASCII_code_repr;
use crate::section_0027::a_open_out;
use crate::section_0054::log_only;
use crate::section_0057::print_ln;
use crate::section_0059::print;
use crate::section_0062::print_nl;
use crate::section_0525::a_make_name_string;
use crate::section_0529::pack_job_name;
