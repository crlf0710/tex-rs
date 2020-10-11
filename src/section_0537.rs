//! @ Let's turn now to the procedure that is used to initiate file reading
//! when an `\.{\\input}' command is being processed.
//
// @p procedure start_input; {\TeX\ will \.{\\input} something}
/// `TeX` will `\input` something
#[allow(unused_variables)]
pub(crate) fn start_input(globals: &mut TeXGlobals) {
    // label done;
    // begin scan_file_name; {set |cur_name| to desired file name}
    // if cur_ext="" then cur_ext:=".tex";
    // pack_cur_name;
    // loop@+  begin begin_file_reading; {set up |cur_file| and new level of input}
    //   if a_open_in(cur_file) then goto done;
    //   if cur_area="" then
    //     begin pack_file_name(cur_name,TEX_area,cur_ext);
    //     if a_open_in(cur_file) then goto done;
    //     end;
    //   end_file_reading; {remove the level that didn't work}
    //   prompt_file_name("input file name",".tex");
    //   end;
    // done: name:=a_make_name_string(cur_file);
    // if job_name=0 then
    //   begin job_name:=cur_name; open_log_file;
    //   end; {|open_log_file| doesn't |show_context|, so |limit|
    //     and |loc| needn't be set to meaningful values yet}
    // if term_offset+length(name)>max_print_line-2 then print_ln
    // else if (term_offset>0)or(file_offset>0) then print_char(" ");
    // print_char("("); incr(open_parens); slow_print(name); update_terminal;
    // state:=new_line;
    // if name=str_ptr-1 then {we can conserve string pool space now}
    //   begin flush_string; name:=cur_name;
    //   end;
    // @<Read the first line of the new file@>;
    Read_the_first_line_of_the_new_file!(globals);
    // end;
}

use crate::section_0004::TeXGlobals;
