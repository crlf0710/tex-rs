//! @ Let's turn now to the procedure that is used to initiate file reading
//! when an `\.{\\input}' command is being processed.
//! Beware: For historic reasons, this code foolishly conserves a tiny bit
//! of string pool space; but that can confuse the interactive `\.E' option.
//! @^system dependencies@>
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
    if globals.cur_ext == crate::strpool_str!("") {
        globals.cur_ext = crate::strpool_str!(".tex");
    }
    // pack_cur_name;
    pack_cur_name(globals);
    crate::region_forward_label! {
    |'done|
    {
    // loop@+  begin begin_file_reading; {set up |cur_file| and new level of input}
    loop {
        /// set up `cur_file` and new level of input
        begin_file_reading(globals);
        // if a_open_in(cur_file) then goto done;
        if a_open_in(make_globals_filename_view!(globals), &mut cur_file!(globals)) {
            crate::goto_forward_label!('done);
        }
        // if cur_area="" then
        if globals.cur_area == crate::strpool_str!("") {
            // begin pack_file_name(cur_name,TEX_area,cur_ext);
            pack_file_name(globals, globals.cur_name, TEX_area!(), globals.cur_ext);
            // if a_open_in(cur_file) then goto done;
            if a_open_in(make_globals_filename_view!(globals), &mut cur_file!(globals)) {
                crate::goto_forward_label!('done);
            }
            // end;
        }
        // end_file_reading; {remove the level that didn't work}
        /// remove the level that didn't work
        end_file_reading(globals);
        // prompt_file_name("input file name",".tex");
        prompt_file_name(globals, crate::strpool_str!("input file name"), crate::strpool_str!(".tex"))?;
        // end;
    }
    }
    'done <-
    }
    // done: name:=a_make_name_string(cur_file);
    name!(globals) = a_make_name_string(
        make_globals_io_string_view!(globals),
        &mut cur_file!(globals),
    )
    .get() as _;
    crate::trace_expr_verbose!("name = {}", name!(globals));
    // if job_name=0 then
    if globals.job_name == 0 {
        // begin job_name:=cur_name; open_log_file;
        globals.job_name = globals.cur_name;
        open_log_file(globals);
        // end; {|open_log_file| doesn't |show_context|, so |limit|
        //   and |loc| needn't be set to meaningful values yet}
        /// `open_log_file` doesn't `show_context`, so `limit`
        /// and `loc` needn't be set to meaningful values yet
        const _: () = ();
    }
    // if term_offset+length(name)>max_print_line-2 then print_ln
    if globals.term_offset.get() as integer + length(globals, name!(globals) as _)
        > globals.max_print_line as integer - 2
    {
        print_ln(make_globals_io_string_log_view!(globals));
    }
    // else if (term_offset>0)or(file_offset>0) then print_char(" ");
    else if globals.term_offset > 0 || globals.file_offset > 0 {
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b' '),
        );
    }
    // print_char("("); incr(open_parens); slow_print(name); update_terminal;
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b'('),
    );
    incr!(globals.open_parens);
    crate::trace_expr_verbose!("open_parens = {:?}", globals.open_parens);
    slow_print(globals, name!(globals).into());
    update_terminal(globals);
    // state:=new_line;
    state!(globals) = new_line;
    // if name=str_ptr-1 then {conserve string pool space (but see note above)}
    if name!(globals) as integer == globals.str_ptr.get() as integer - 1 {
        /// conserve string pool space (but see note above)
        const _: () = ();
        // begin flush_string; name:=cur_name;
        flush_string(globals);
        name!(globals) = globals.cur_name.get() as _;
        // end;
    }
    // @<Read the first line of the new file@>;
    crate::section_0538::Read_the_first_line_of_the_new_file!(globals);
    // end;
    crate::return_nojump!();
}

use crate::pascal::integer;
use crate::section_0004::make_globals_filename_view;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::make_globals_io_string_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsFilenameView;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0004::TeXGlobalsIoStringView;
use crate::section_0016::incr;
use crate::section_0018::ASCII_code_literal;
use crate::section_0027::a_open_in;
use crate::section_0034::update_terminal;
use crate::section_0040::length;
use crate::section_0044::flush_string;
use crate::section_0057::print_ln;
use crate::section_0058::print_char;
use crate::section_0060::slow_print;
use crate::section_0081::TeXResult;
use crate::section_0302::name;
use crate::section_0302::state;
use crate::section_0303::new_line;
use crate::section_0304::cur_file;
use crate::section_0328::begin_file_reading;
use crate::section_0329::end_file_reading;
use crate::section_0514::TEX_area;
use crate::section_0519::pack_file_name;
use crate::section_0525::a_make_name_string;
use crate::section_0526::scan_file_name;
use crate::section_0529::pack_cur_name;
use crate::section_0530::prompt_file_name;
use crate::section_0534::open_log_file;
