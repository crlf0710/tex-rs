//! @ Here's an example of how these conventions are used. Whenever it is time to
//! ship out a box of stuff, we shall use the macro |ensure_dvi_open|.
//!
//! @d ensure_dvi_open==if output_file_name=0 then
//!   begin if job_name=0 then open_log_file;
//!   pack_job_name(".dvi");
//!   while not b_open_out(dvi_file) do
//!     prompt_file_name("file name for output",".dvi");
//!   output_file_name:=b_make_name_string(dvi_file);
//!   end
//!
//! @<Glob...@>=
//! @!dvi_file: byte_file; {the device-independent output goes here}
//! @!output_file_name: str_number; {full name of the output file}
//! @!log_name:str_number; {full name of the log file}
//!
//! @ @<Initialize the output...@>=output_file_name:=0;
//!
//! @ The |open_log_file| routine is used to open the transcript file and to help
//! it catch up to what has previously been printed on the terminal.
//!
//! @p procedure open_log_file;
//! var old_setting:0..max_selector; {previous |selector| setting}
//! @!k:0..buf_size; {index into |months| and |buffer|}
//! @!l:0..buf_size; {end of first input line}
//! @!months:packed array [1..36] of char; {abbreviations of month names}
//! begin old_setting:=selector;
//! if job_name=0 then job_name:="texput";
//! @.texput@>
//! pack_job_name(".log");
//! while not a_open_out(log_file) do @<Try to get a different log file name@>;
//! log_name:=a_make_name_string(log_file);
//! selector:=log_only; log_opened:=true;
//! @<Print the banner line, including the date and time@>;
//! input_stack[input_ptr]:=cur_input; {make sure bottom level is in memory}
//! print_nl("**");
//! @.**@>
//! l:=input_stack[0].limit_field; {last position of first line}
//! if buffer[l]=end_line_char then decr(l);
//! for k:=1 to l do print(buffer[k]);
//! print_ln; {now the transcript file contains the first line of input}
//! selector:=old_setting+2; {|log_only| or |term_and_log|}
//! end;
//!
//! @ Sometimes |open_log_file| is called at awkward moments when \TeX\ is
//! unable to print error messages or even to |show_context|.
//! The |prompt_file_name| routine can result in a |fatal_error|, but the |error|
//! routine will not be invoked because |log_opened| will be false.
//!
//! The normal idea of |batch_mode| is that nothing at all should be written
//! on the terminal. However, in the unusual case that
//! no log file could be opened, we make an exception and allow
//! an explanatory message to be seen.
//!
//! Incidentally, the program always refers to the log file as a `\.{transcript
//! file}', because some systems cannot use the extension `\.{.log}' for
//! this file.
//!
//! @<Try to get a different log file name@>=
//! begin selector:=term_only;
//! prompt_file_name("transcript file name",".log");
//! end
//!
//! @ @<Print the banner...@>=
//! begin wlog(banner);
//! slow_print(format_ident); print("  ");
//! print_int(day); print_char(" ");
//! months:='JANFEBMARAPRMAYJUNJULAUGSEPOCTNOVDEC';
//! for k:=3*month-2 to 3*month do wlog(months[k]);
//! print_char(" "); print_int(year); print_char(" ");
//! print_two(time div 60); print_char(":"); print_two(time mod 60);
//! end
//!
