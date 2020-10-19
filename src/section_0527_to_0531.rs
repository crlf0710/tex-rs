//! @ The global variable |name_in_progress| is used to prevent recursive
//! use of |scan_file_name|, since the |begin_name| and other procedures
//! communicate via global variables. Recursion would arise only by
//! devious tricks like `\.{\\input\\input f}'; such attempts at sabotage
//! must be thwarted. Furthermore, |name_in_progress| prevents \.{\\input}
//! @^recursion@>
//! from being initiated when a font size specification is being scanned.
//!
//! Another global variable, |job_name|, contains the file name that was first
//! \.{\\input} by the user. This name is extended by `\.{.log}' and `\.{.dvi}'
//! and `\.{.fmt}' in the names of \TeX's output files.
//!
//! @<Glob...@>=
//! @!name_in_progress:boolean; {is a file name being scanned?}
//! @!job_name:str_number; {principal file name}
//! @!log_opened:boolean; {has the transcript file been opened?}
//!
//! @ Initially |job_name=0|; it becomes nonzero as soon as the true name is known.
//! We have |job_name=0| if and only if the `\.{log}' file has not been opened,
//! except of course for a short time just after |job_name| has become nonzero.
//!
//! @<Initialize the output...@>=
//! job_name:=0; name_in_progress:=false; log_opened:=false;
//!
//! @ Here is a routine that manufactures the output file names, assuming that
//! |job_name<>0|. It ignores and changes the current settings of |cur_area|
//! and |cur_ext|.
//!
//! @d pack_cur_name==pack_file_name(cur_name,cur_area,cur_ext)
//!
//! @p procedure pack_job_name(@!s:str_number); {|s = ".log"|, |".dvi"|, or
//!   |format_extension|}
//! begin cur_area:=""; cur_ext:=s;
//! cur_name:=job_name; pack_cur_name;
//! end;
//!
//! @ If some trouble arises when \TeX\ tries to open a file, the following
//! routine calls upon the user to supply another file name. Parameter~|s|
//! is used in the error message to identify the type of file; parameter~|e|
//! is the default extension if none is given. Upon exit from the routine,
//! variables |cur_name|, |cur_area|, |cur_ext|, and |name_of_file| are
//! ready for another attempt at file opening.
//!
//! @p procedure prompt_file_name(@!s,@!e:str_number);
//! label done;
//! var k:0..buf_size; {index into |buffer|}
//! begin if interaction=scroll_mode then wake_up_terminal;
//! if s="input file name" then print_err("I can't find file `")
//! @.I can't find file x@>
//! else print_err("I can't write on file `");
//! @.I can't write on file x@>
//! print_file_name(cur_name,cur_area,cur_ext); print("'.");
//! if e=".tex" then show_context;
//! print_nl("Please type another "); print(s);
//! @.Please type...@>
//! if interaction<scroll_mode then
//!   fatal_error("*** (job aborted, file error in nonstop mode)");
//! @.job aborted, file error...@>
//! clear_terminal; prompt_input(": "); @<Scan file name in the buffer@>;
//! if cur_ext="" then cur_ext:=e;
//! pack_cur_name;
//! end;
//!
//! @ @<Scan file name in the buffer@>=
//! begin begin_name; k:=first;
//! while (buffer[k]=" ")and(k<last) do incr(k);
//! loop@+  begin if k=last then goto done;
//!   if not more_name(buffer[k]) then goto done;
//!   incr(k);
//!   end;
//! done:end_name;
//! end
//!
