//! @ Let's turn now to the procedure that is used to initiate file reading
//! when an `\.{\\input}' command is being processed.
//
// @p procedure start_input; {\TeX\ will \.{\\input} something}
/// `TeX` will `\input` something
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn start_input(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label done;
    // begin scan_file_name; {set |cur_name| to desired file name}
    /// set `cur_name` to desired file name
    scan_file_name(globals)?;
    // if cur_ext="" then cur_ext:=".tex";
    if globals.cur_ext == strpool_str!("") {
        globals.cur_ext = strpool_str!(".tex");
    }
    // pack_cur_name;
    pack_cur_name(globals);
    region_forward_label! {
    |'done|
    {
    // loop@+  begin begin_file_reading; {set up |cur_file| and new level of input}
    loop {
        /// set up `cur_file` and new level of input
        begin_file_reading(globals);
        // if a_open_in(cur_file) then goto done;
        if a_open_in(make_globals_filename_view!(globals), &mut cur_file!(globals)) {
            goto_forward_label!('done);
        }
        // if cur_area="" then
        //   begin pack_file_name(cur_name,TEX_area,cur_ext);
        //   if a_open_in(cur_file) then goto done;
        if a_open_in(make_globals_filename_view!(globals), &mut cur_file!(globals)) {
            goto_forward_label!('done);
        }
        //   end;
        // end_file_reading; {remove the level that didn't work}
        /// remove the level that didn't work
        end_file_reading(globals);
        prompt_file_name(globals, strpool_str!("input file name"),strpool_str!(".tex"))?;
        // end;
    }
    }
    'done <-
    }
    // done: name:=a_make_name_string(cur_file);
    name!(globals) = a_make_name_string(make_globals_io_string_view!(globals), &mut cur_file!(globals)).get() as _;
    trace_expr!("name = {}", name!(globals));
    // if job_name=0 then
    //   begin job_name:=cur_name; open_log_file;
    //   end; {|open_log_file| doesn't |show_context|, so |limit|
    //     and |loc| needn't be set to meaningful values yet}
    // if term_offset+length(name)>max_print_line-2 then print_ln
    // else if (term_offset>0)or(file_offset>0) then print_char(" ");
    // print_char("("); incr(open_parens); slow_print(name); update_terminal;
    print_char(make_globals_io_string_log_view!(globals), ASCII_code_literal!(b'('));
    incr!(globals.open_parens);
    trace_expr!("open_parens = {:?}", globals.open_parens);
    slow_print(globals, name!(globals).into());
    update_terminal(globals);
    // state:=new_line;
    state!(globals) = new_line;
    // if name=str_ptr-1 then {we can conserve string pool space now}
    //   begin flush_string; name:=cur_name;
    //   end;
    // @<Read the first line of the new file@>;
    Read_the_first_line_of_the_new_file!(globals);
    // end;
    return_nojump!();
}

use crate::section_0004::{TeXGlobals, TeXGlobalsFilenameView, TeXGlobalsIoStringView, TeXGlobalsIoStringLogView};
use crate::section_0034::update_terminal;
use crate::section_0526::scan_file_name;
use crate::section_0058::print_char;
use crate::section_0060::slow_print;
use crate::section_0303::new_line;
use crate::section_0328::begin_file_reading;
use crate::section_0329::end_file_reading;
use crate::section_0027::a_open_in;
use crate::section_0529::pack_cur_name;
use crate::section_0530::prompt_file_name;
use crate::section_0081::TeXResult;
use crate::section_0525::a_make_name_string;