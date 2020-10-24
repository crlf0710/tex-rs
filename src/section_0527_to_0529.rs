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
