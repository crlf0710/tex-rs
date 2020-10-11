//! @* \[29] File names.
//! It's time now to fret about file names.  Besides the fact that different
//! operating systems treat files in different ways, we must cope with the
//! fact that completely different naming conventions are used by different
//! groups of people. The following programs show what is required for one
//! particular operating system; similar routines for other systems are not
//! difficult to devise.
//! @^fingers@>
//! @^system dependencies@>
//!
//! \TeX\ assumes that a file name has three parts: the name proper; its
//! ``extension''; and a ``file area'' where it is found in an external file
//! system.  The extension of an input file or a write file is assumed to be
//! `\.{.tex}' unless otherwise specified; it is `\.{.log}' on the
//! transcript file that records each run of \TeX; it is `\.{.tfm}' on the font
//! metric files that describe characters in the fonts \TeX\ uses; it is
//! `\.{.dvi}' on the output files that specify typesetting information; and it
//! is `\.{.fmt}' on the format files written by \.{INITEX} to initialize \TeX.
//! The file area can be arbitrary on input files, but files are usually
//! output to the user's current area.  If an input file cannot be
//! found on the specified area, \TeX\ will look for it on a special system
//! area; this special area is intended for commonly used input files like
//! \.{webmac.tex}.
//!
//! Simple uses of \TeX\ refer only to file names that have no explicit
//! extension or area. For example, a person usually says `\.{\\input} \.{paper}'
//! or `\.{\\font\\tenrm} \.= \.{helvetica}' instead of `\.{\\input}
//! \.{paper.new}' or `\.{\\font\\tenrm} \.= \.{<csd.knuth>test}'. Simple file
//! names are best, because they make the \TeX\ source files portable;
//! whenever a file name consists entirely of letters and digits, it should be
//! treated in the same way by all implementations of \TeX. However, users
//! need the ability to refer to other files in their environment, especially
//! when responding to error messages concerning unopenable files; therefore
//! we want to let them use the syntax that appears in their favorite
//! operating system.
//!
//! The following procedures don't allow spaces to be part of
//! file names; but some users seem to like names that are spaced-out.
//! System-dependent changes to allow such things should probably
//! be made with reluctance, and only when an entire file name that
//! includes spaces is ``quoted'' somehow.
//!
//! @ In order to isolate the system-dependent aspects of file names, the
//! @^system dependencies@>
//! system-independent parts of \TeX\ are expressed in terms
//! of three system-dependent
//! procedures called |begin_name|, |more_name|, and |end_name|. In
//! essence, if the user-specified characters of the file name are $c_1\ldots c_n$,
//! the system-independent driver program does the operations
//! $$|begin_name|;\,|more_name|(c_1);\,\ldots\,;\,|more_name|(c_n);
//! \,|end_name|.$$
//! These three procedures communicate with each other via global variables.
//! Afterwards the file name will appear in the string pool as three strings
//! called |cur_name|\penalty10000\hskip-.05em,
//! |cur_area|, and |cur_ext|; the latter two are null (i.e.,
//! |""|), unless they were explicitly specified by the user.
//!
//! Actually the situation is slightly more complicated, because \TeX\ needs
//! to know when the file name ends. The |more_name| routine is a function
//! (with side effects) that returns |true| on the calls |more_name|$(c_1)$,
//! \dots, |more_name|$(c_{n-1})$. The final call |more_name|$(c_n)$
//! returns |false|; or, it returns |true| and the token following $c_n$ is
//! something like `\.{\\hbox}' (i.e., not a character). In other words,
//! |more_name| is supposed to return |true| unless it is sure that the
//! file name has been completely scanned; and |end_name| is supposed to be able
//! to finish the assembly of |cur_name|, |cur_area|, and |cur_ext| regardless of
//! whether $|more_name|(c_n)$ returned |true| or |false|.
//!
//! @<Glob...@>=
//! @!cur_name:str_number; {name of file just scanned}
//! @!cur_area:str_number; {file area just scanned, or \.{""}}
//! @!cur_ext:str_number; {file extension just scanned, or \.{""}}
//!
//! @ The file names we shall deal with for illustrative purposes have the
//! following structure:  If the name contains `\.>' or `\.:', the file area
//! consists of all characters up to and including the final such character;
//! otherwise the file area is null.  If the remaining file name contains
//! `\..', the file extension consists of all such characters from the first
//! remaining `\..' to the end, otherwise the file extension is null.
//! @^system dependencies@>
//!
//! We can scan such file names easily by using two global variables that keep track
//! of the occurrences of area and extension delimiters:
//!
//! @<Glob...@>=
//! @!area_delimiter:pool_pointer; {the most recent `\.>' or `\.:', if any}
//! @!ext_delimiter:pool_pointer; {the relevant `\..', if any}
//!
//! @ Input files that can't be found in the user's area may appear in a standard
//! system area called |TEX_area|. Font metric files whose areas are not given
//! explicitly are assumed to appear in a standard system area called
//! |TEX_font_area|.  These system area names will, of course, vary from place
//! to place.
//! @^system dependencies@>
//!
//! @d TEX_area=="TeXinputs:"
//! @.TeXinputs@>
//! @d TEX_font_area=="TeXfonts:"
//! @.TeXfonts@>
//!
//! @ Here now is the first of the system-dependent routines for file name scanning.
//! @^system dependencies@>
//!
//! @p procedure begin_name;
//! begin area_delimiter:=0; ext_delimiter:=0;
//! end;
//!
//! @ And here's the second. The string pool might change as the file name is
//! being scanned, since a new \.{\\csname} might be entered; therefore we keep
//! |area_delimiter| and |ext_delimiter| relative to the beginning of the current
//! string, instead of assigning an absolute address like |pool_ptr| to them.
//! @^system dependencies@>
//!
//! @p function more_name(@!c:ASCII_code):boolean;
//! begin if c=" " then more_name:=false
//! else  begin str_room(1); append_char(c); {contribute |c| to the current string}
//!   if (c=">")or(c=":") then
//!     begin area_delimiter:=cur_length; ext_delimiter:=0;
//!     end
//!   else if (c=".")and(ext_delimiter=0) then ext_delimiter:=cur_length;
//!   more_name:=true;
//!   end;
//! end;
//!
//! @ The third.
//! @^system dependencies@>
//!
//! @p procedure end_name;
//! begin if str_ptr+3>max_strings then
//!   overflow("number of strings",max_strings-init_str_ptr);
//! @:TeX capacity exceeded number of strings}{\quad number of strings@>
//! if area_delimiter=0 then cur_area:=""
//! else  begin cur_area:=str_ptr;
//!   str_start[str_ptr+1]:=str_start[str_ptr]+area_delimiter; incr(str_ptr);
//!   end;
//! if ext_delimiter=0 then
//!   begin cur_ext:=""; cur_name:=make_string;
//!   end
//! else  begin cur_name:=str_ptr;
//!   str_start[str_ptr+1]:=str_start[str_ptr]+ext_delimiter-area_delimiter-1;
//!   incr(str_ptr); cur_ext:=make_string;
//!   end;
//! end;
//!
//! @ Conversely, here is a routine that takes three strings and prints a file
//! name that might have produced them. (The routine is system dependent, because
//! some operating systems put the file area last instead of first.)
//! @^system dependencies@>
//!
//! @<Basic printing...@>=
//! procedure print_file_name(@!n,@!a,@!e:integer);
//! begin slow_print(a); slow_print(n); slow_print(e);
//! end;
//!
//! @ Another system-dependent routine is needed to convert three internal
//! \TeX\ strings
//! into the |name_of_file| value that is used to open files. The present code
//! allows both lowercase and uppercase letters in the file name.
//! @^system dependencies@>
//!
//! @d append_to_name(#)==begin c:=#; incr(k);
//!   if k<=file_name_size then name_of_file[k]:=xchr[c];
//!   end
//!
//! @p procedure pack_file_name(@!n,@!a,@!e:str_number);
//! var k:integer; {number of positions filled in |name_of_file|}
//! @!c: ASCII_code; {character being packed}
//! @!j:pool_pointer; {index into |str_pool|}
//! begin k:=0;
//! for j:=str_start[a] to str_start[a+1]-1 do append_to_name(so(str_pool[j]));
//! for j:=str_start[n] to str_start[n+1]-1 do append_to_name(so(str_pool[j]));
//! for j:=str_start[e] to str_start[e+1]-1 do append_to_name(so(str_pool[j]));
//! if k<=file_name_size then name_length:=k@+else name_length:=file_name_size;
//! for k:=name_length+1 to file_name_size do name_of_file[k]:=' ';
//! end;
//!
//! @ A messier routine is also needed, since format file names must be scanned
//! before \TeX's string mechanism has been initialized. We shall use the
//! global variable |TEX_format_default| to supply the text for default system areas
//! and extensions related to format files.
//! @^system dependencies@>
//!
//! @d format_default_length=20 {length of the |TEX_format_default| string}
//! @d format_area_length=11 {length of its area part}
//! @d format_ext_length=4 {length of its `\.{.fmt}' part}
//! @d format_extension=".fmt" {the extension, as a \.{WEB} constant}
//!
//! @<Glob...@>=
//! @!TEX_format_default:packed array[1..format_default_length] of char;
//!
//! @ @<Set init...@>=
//! TEX_format_default:='TeXformats:plain.fmt';
//! @.TeXformats@>
//! @.plain@>
//! @^system dependencies@>
//!
//! @ @<Check the ``constant'' values for consistency@>=
//! if format_default_length>file_name_size then bad:=31;
//!
//! @ Here is the messy routine that was just mentioned. It sets |name_of_file|
//! from the first |n| characters of |TEX_format_default|, followed by
//! |buffer[a..b]|, followed by the last |format_ext_length| characters of
//! |TEX_format_default|.
//!
//! We dare not give error messages here, since \TeX\ calls this routine before
//! the |error| routine is ready to roll. Instead, we simply drop excess characters,
//! since the error will be detected in another way when a strange file name
//! isn't found.
//! @^system dependencies@>
//!
//! @p procedure pack_buffered_name(@!n:small_number;@!a,@!b:integer);
//! var k:integer; {number of positions filled in |name_of_file|}
//! @!c: ASCII_code; {character being packed}
//! @!j:integer; {index into |buffer| or |TEX_format_default|}
//! begin if n+b-a+1+format_ext_length>file_name_size then
//!   b:=a+file_name_size-n-1-format_ext_length;
//! k:=0;
//! for j:=1 to n do append_to_name(xord[TEX_format_default[j]]);
//! for j:=a to b do append_to_name(buffer[j]);
//! for j:=format_default_length-format_ext_length+1 to format_default_length do
//!   append_to_name(xord[TEX_format_default[j]]);
//! if k<=file_name_size then name_length:=k@+else name_length:=file_name_size;
//! for k:=name_length+1 to file_name_size do name_of_file[k]:=' ';
//! end;
//!
//! @ Here is the only place we use |pack_buffered_name|. This part of the program
//! becomes active when a ``virgin'' \TeX\ is trying to get going, just after
//! the preliminary initialization, or when the user is substituting another
//! format file by typing `\.\&' after the initial `\.{**}' prompt.  The buffer
//! contains the first line of input in |buffer[loc..(last-1)]|, where
//! |loc<last| and |buffer[loc]<>" "|.
//!
//! @<Declare the function called |open_fmt_file|@>=
//! function open_fmt_file:boolean;
//! label found,exit;
//! var j:0..buf_size; {the first space after the format file name}
//! begin j:=loc;
//! if buffer[loc]="&" then
//!   begin incr(loc); j:=loc; buffer[last]:=" ";
//!   while buffer[j]<>" " do incr(j);
//!   pack_buffered_name(0,loc,j-1); {try first without the system file area}
//!   if w_open_in(fmt_file) then goto found;
//!   pack_buffered_name(format_area_length,loc,j-1);
//!     {now try the system format file area}
//!   if w_open_in(fmt_file) then goto found;
//!   wake_up_terminal;
//!   wterm_ln('Sorry, I can''t find that format;',' will try PLAIN.');
//! @.Sorry, I can't find...@>
//!   update_terminal;
//!   end;
//!   {now pull out all the stops: try for the system \.{plain} file}
//! pack_buffered_name(format_default_length-format_ext_length,1,0);
//! if not w_open_in(fmt_file) then
//!   begin wake_up_terminal;
//!   wterm_ln('I can''t find the PLAIN format file!');
//! @.I can't find PLAIN...@>
//! @.plain@>
//!   open_fmt_file:=false; return;
//!   end;
//! found:loc:=j; open_fmt_file:=true;
//! exit:end;
//!
//! @ Operating systems often make it possible to determine the exact name (and
//! possible version number) of a file that has been opened. The following routine,
//! which simply makes a \TeX\ string from the value of |name_of_file|, should
//! ideally be changed to deduce the full name of file~|f|, which is the file
//! most recently opened, if it is possible to do this in a \PASCAL\ program.
//! @^system dependencies@>
//!
//! This routine might be called after string memory has overflowed, hence
//! we dare not use `|str_room|'.
//!
//! @p function make_name_string:str_number;
//! var k:1..file_name_size; {index into |name_of_file|}
//! begin if (pool_ptr+name_length>pool_size)or(str_ptr=max_strings)or
//!  (cur_length>0) then
//!   make_name_string:="?"
//! else  begin for k:=1 to name_length do append_char(xord[name_of_file[k]]);
//!   make_name_string:=make_string;
//!   end;
//! end;
//! function a_make_name_string(var f:alpha_file):str_number;
//! begin a_make_name_string:=make_name_string;
//! end;
//! function b_make_name_string(var f:byte_file):str_number;
//! begin b_make_name_string:=make_name_string;
//! end;
//! function w_make_name_string(var f:word_file):str_number;
//! begin w_make_name_string:=make_name_string;
//! end;
//!
//! @ Now let's consider the ``driver''
//! routines by which \TeX\ deals with file names
//! in a system-independent manner.  First comes a procedure that looks for a
//! file name in the input by calling |get_x_token| for the information.
//!
//! @p procedure scan_file_name;
//! label done;
//! begin name_in_progress:=true; begin_name;
//! @<Get the next non-blank non-call...@>;
//! loop@+begin if (cur_cmd>other_char)or(cur_chr>255) then {not a character}
//!     begin back_input; goto done;
//!     end;
//!   if not more_name(cur_chr) then goto done;
//!   get_x_token;
//!   end;
//! done: end_name; name_in_progress:=false;
//! end;
//!
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
