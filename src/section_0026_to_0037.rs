//! @ Most of what we need to do with respect to input and output can be handled
//! by the I/O facilities that are standard in \PASCAL, i.e., the routines
//! called |get|, |put|, |eof|, and so on. But
//! standard \PASCAL\ does not allow file variables to be associated with file
//! names that are determined at run time, so it cannot be used to implement
//! \TeX; some sort of extension to \PASCAL's ordinary |reset| and |rewrite|
//! is crucial for our purposes. We shall assume that |name_of_file| is a variable
//! of an appropriate type such that the \PASCAL\ run-time system being used to
//! implement \TeX\ can open a file whose external name is specified by
//! |name_of_file|.
//! @^system dependencies@>
//!
//! @<Glob...@>=
//! @!name_of_file:packed array[1..file_name_size] of char;@;@/
//!   {on some systems this may be a \&{record} variable}
//! @!name_length:0..file_name_size;@/{this many characters are actually
//!   relevant in |name_of_file| (the rest are blank)}
//!
//! @ The \ph\ compiler with which the present version of \TeX\ was prepared has
//! extended the rules of \PASCAL\ in a very convenient way. To open file~|f|,
//! we can write
//! $$\vbox{\halign{#\hfil\qquad&#\hfil\cr
//! |reset(f,@t\\{name}@>,'/O')|&for input;\cr
//! |rewrite(f,@t\\{name}@>,'/O')|&for output.\cr}}$$
//! The `\\{name}' parameter, which is of type `{\bf packed array
//! $[\langle\\{any}\rangle]$ of \\{char}}', stands for the name of
//! the external file that is being opened for input or output.
//! Blank spaces that might appear in \\{name} are ignored.
//!
//! The `\.{/O}' parameter tells the operating system not to issue its own
//! error messages if something goes wrong. If a file of the specified name
//! cannot be found, or if such a file cannot be opened for some other reason
//! (e.g., someone may already be trying to write the same file), we will have
//! |@!erstat(f)<>0| after an unsuccessful |reset| or |rewrite|.  This allows
//! \TeX\ to undertake appropriate corrective action.
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>
//!
//! \TeX's file-opening procedures return |false| if no file identified by
//! |name_of_file| could be opened.
//!
//! @d reset_OK(#)==erstat(#)=0
//! @d rewrite_OK(#)==erstat(#)=0
//!
//! @p function a_open_in(var f:alpha_file):boolean;
//!   {open a text file for input}
//! begin reset(f,name_of_file,'/O'); a_open_in:=reset_OK(f);
//! end;
//! @#
//! function a_open_out(var f:alpha_file):boolean;
//!   {open a text file for output}
//! begin rewrite(f,name_of_file,'/O'); a_open_out:=rewrite_OK(f);
//! end;
//! @#
//! function b_open_in(var f:byte_file):boolean;
//!   {open a binary file for input}
//! begin reset(f,name_of_file,'/O'); b_open_in:=reset_OK(f);
//! end;
//! @#
//! function b_open_out(var f:byte_file):boolean;
//!   {open a binary file for output}
//! begin rewrite(f,name_of_file,'/O'); b_open_out:=rewrite_OK(f);
//! end;
//! @#
//! function w_open_in(var f:word_file):boolean;
//!   {open a word file for input}
//! begin reset(f,name_of_file,'/O'); w_open_in:=reset_OK(f);
//! end;
//! @#
//! function w_open_out(var f:word_file):boolean;
//!   {open a word file for output}
//! begin rewrite(f,name_of_file,'/O'); w_open_out:=rewrite_OK(f);
//! end;
//!
//! @ Files can be closed with the \ph\ routine `|close(f)|', which
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>
//! should be used when all input or output with respect to |f| has been completed.
//! This makes |f| available to be opened again, if desired; and if |f| was used for
//! output, the |close| operation makes the corresponding external file appear
//! on the user's area, ready to be read.
//!
//! These procedures should not generate error messages if a file is
//! being closed before it has been successfully opened.
//!
//! @p procedure a_close(var f:alpha_file); {close a text file}
//! begin close(f);
//! end;
//! @#
//! procedure b_close(var f:byte_file); {close a binary file}
//! begin close(f);
//! end;
//! @#
//! procedure w_close(var f:word_file); {close a word file}
//! begin close(f);
//! end;
//!
//! @ Binary input and output are done with \PASCAL's ordinary |get| and |put|
//! procedures, so we don't have to make any other special arrangements for
//! binary~I/O. Text output is also easy to do with standard \PASCAL\ routines.
//! The treatment of text input is more difficult, however, because
//! of the necessary translation to |ASCII_code| values.
//! \TeX's conventions should be efficient, and they should
//! blend nicely with the user's operating environment.
//!
//! @ Input from text files is read one line at a time, using a routine called
//! |input_ln|. This function is defined in terms of global variables called
//! |buffer|, |first|, and |last| that will be described in detail later; for
//! now, it suffices for us to know that |buffer| is an array of |ASCII_code|
//! values, and that |first| and |last| are indices into this array
//! representing the beginning and ending of a line of text.
//!
//! @<Glob...@>=
//! @!buffer:array[0..buf_size] of ASCII_code; {lines of characters being read}
//! @!first:0..buf_size; {the first unused position in |buffer|}
//! @!last:0..buf_size; {end of the line just input to |buffer|}
//! @!max_buf_stack:0..buf_size; {largest index used in |buffer|}
//!
//! @ The |input_ln| function brings the next line of input from the specified
//! file into available positions of the buffer array and returns the value
//! |true|, unless the file has already been entirely read, in which case it
//! returns |false| and sets |last:=first|.  In general, the |ASCII_code|
//! numbers that represent the next line of the file are input into
//! |buffer[first]|, |buffer[first+1]|, \dots, |buffer[last-1]|; and the
//! global variable |last| is set equal to |first| plus the length of the
//! line. Trailing blanks are removed from the line; thus, either |last=first|
//! (in which case the line was entirely blank) or |buffer[last-1]<>" "|.
//!
//! An overflow error is given, however, if the normal actions of |input_ln|
//! would make |last>=buf_size|; this is done so that other parts of \TeX\
//! can safely look at the contents of |buffer[last+1]| without overstepping
//! the bounds of the |buffer| array. Upon entry to |input_ln|, the condition
//! |first<buf_size| will always hold, so that there is always room for an
//! ``empty'' line.
//!
//! The variable |max_buf_stack|, which is used to keep track of how large
//! the |buf_size| parameter must be to accommodate the present job, is
//! also kept up to date by |input_ln|.
//!
//! If the |bypass_eoln| parameter is |true|, |input_ln| will do a |get|
//! before looking at the first character of the line; this skips over
//! an |eoln| that was in |f^|. The procedure does not do a |get| when it
//! reaches the end of the line; therefore it can be used to acquire input
//! from the user's terminal as well as from ordinary text files.
//!
//! Standard \PASCAL\ says that a file should have |eoln| immediately
//! before |eof|, but \TeX\ needs only a weaker restriction: If |eof|
//! occurs in the middle of a line, the system function |eoln| should return
//! a |true| result (even though |f^| will be undefined).
//!
//! Since the inner loop of |input_ln| is part of \TeX's ``inner loop''---each
//! character of input comes in at this place---it is wise to reduce system
//! overhead by making use of special routines that read in an entire array
//! of characters at once, if such routines are available. The following
//! code uses standard \PASCAL\ to illustrate what needs to be done, but
//! finer tuning is often possible at well-developed \PASCAL\ sites.
//! @^inner loop@>
//!
//! @p function input_ln(var f:alpha_file;@!bypass_eoln:boolean):boolean;
//!   {inputs the next line or returns |false|}
//! var last_nonblank:0..buf_size; {|last| with trailing blanks removed}
//! begin if bypass_eoln then if not eof(f) then get(f);
//!   {input the first character of the line into |f^|}
//! last:=first; {cf.\ Matthew 19\thinspace:\thinspace30}
//! if eof(f) then input_ln:=false
//! else  begin last_nonblank:=first;
//!   while not eoln(f) do
//!     begin if last>=max_buf_stack then
//!       begin max_buf_stack:=last+1;
//!       if max_buf_stack=buf_size then
//!         @<Report overflow of the input buffer, and abort@>;
//!       end;
//!     buffer[last]:=xord[f^]; get(f); incr(last);
//!     if buffer[last-1]<>" " then last_nonblank:=last;
//!     end;
//!   last:=last_nonblank; input_ln:=true;
//!   end;
//! end;
//!
//! @ The user's terminal acts essentially like other files of text, except
//! that it is used both for input and for output. When the terminal is
//! considered an input file, the file variable is called |term_in|, and when it
//! is considered an output file the file variable is |term_out|.
//! @^system dependencies@>
//!
//! @<Glob...@>=
//! @!term_in:alpha_file; {the terminal as an input file}
//! @!term_out:alpha_file; {the terminal as an output file}
//!
//! @ Here is how to open the terminal files
//! in \ph. The `\.{/I}' switch suppresses the first |get|.
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>
//!
//! @d t_open_in==reset(term_in,'TTY:','/O/I') {open the terminal for text input}
//! @d t_open_out==rewrite(term_out,'TTY:','/O') {open the terminal for text output}
//!
//! @ Sometimes it is necessary to synchronize the input/output mixture that
//! happens on the user's terminal, and three system-dependent
//! procedures are used for this
//! purpose. The first of these, |update_terminal|, is called when we want
//! to make sure that everything we have output to the terminal so far has
//! actually left the computer's internal buffers and been sent.
//! The second, |clear_terminal|, is called when we wish to cancel any
//! input that the user may have typed ahead (since we are about to
//! issue an unexpected error message). The third, |wake_up_terminal|,
//! is supposed to revive the terminal if the user has disabled it by
//! some instruction to the operating system.  The following macros show how
//! these operations can be specified in \ph:
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>
//!
//! @d update_terminal == break(term_out) {empty the terminal output buffer}
//! @d clear_terminal == break_in(term_in,true) {clear the terminal input buffer}
//! @d wake_up_terminal == do_nothing {cancel the user's cancellation of output}
//!
//! @ We need a special routine to read the first line of \TeX\ input from
//! the user's terminal. This line is different because it is read before we
//! have opened the transcript file; there is sort of a ``chicken and
//! egg'' problem here. If the user types `\.{\\input paper}' on the first
//! line, or if some macro invoked by that line does such an \.{\\input},
//! the transcript file will be named `\.{paper.log}'; but if no \.{\\input}
//! commands are performed during the first line of terminal input, the transcript
//! file will acquire its default name `\.{texput.log}'. (The transcript file
//! will not contain error messages generated by the first line before the
//! first \.{\\input} command.)
//! @.texput@>
//!
//! The first line is even more special if we are lucky enough to have an operating
//! system that treats \TeX\ differently from a run-of-the-mill \PASCAL\ object
//! program. It's nice to let the user start running a \TeX\ job by typing
//! a command line like `\.{tex paper}'; in such a case, \TeX\ will operate
//! as if the first line of input were `\.{paper}', i.e., the first line will
//! consist of the remainder of the command line, after the part that invoked
//! \TeX.
//!
//! The first line is special also because it may be read before \TeX\ has
//! input a format file. In such cases, normal error messages cannot yet
//! be given. The following code uses concepts that will be explained later.
//! (If the \PASCAL\ compiler does not support non-local |@!goto|\unskip, the
//! @^system dependencies@>
//! statement `|goto final_end|' should be replaced by something that
//! quietly terminates the program.)
//!
//! @<Report overflow of the input buffer, and abort@>=
//! if format_ident=0 then
//!   begin write_ln(term_out,'Buffer size exceeded!'); goto final_end;
//! @.Buffer size exceeded@>
//!   end
//! else begin cur_input.loc_field:=first; cur_input.limit_field:=last-1;
//!   overflow("buffer size",buf_size);
//! @:TeX capacity exceeded buffer size}{\quad buffer size@>
//!   end
//!
//! @ Different systems have different ways to get started. But regardless of
//! what conventions are adopted, the routine that initializes the terminal
//! should satisfy the following specifications:
//!
//! \yskip\textindent{1)}It should open file |term_in| for input from the
//!   terminal. (The file |term_out| will already be open for output to the
//!   terminal.)
//!
//! \textindent{2)}If the user has given a command line, this line should be
//!   considered the first line of terminal input. Otherwise the
//!   user should be prompted with `\.{**}', and the first line of input
//!   should be whatever is typed in response.
//!
//! \textindent{3)}The first line of input, which might or might not be a
//!   command line, should appear in locations |first| to |last-1| of the
//!   |buffer| array.
//!
//! \textindent{4)}The global variable |loc| should be set so that the
//!   character to be read next by \TeX\ is in |buffer[loc]|. This
//!   character should not be blank, and we should have |loc<last|.
//!
//! \yskip\noindent(It may be necessary to prompt the user several times
//! before a non-blank line comes in. The prompt is `\.{**}' instead of the
//! later `\.*' because the meaning is slightly different: `\.{\\input}' need
//! not be typed immediately after~`\.{**}'.)
//!
//! @d loc==cur_input.loc_field {location of first unread character in |buffer|}
//!
//! @ The following program does the required initialization
//! without retrieving a possible command line.
//! It should be clear how to modify this routine to deal with command lines,
//! if the system permits them.
//! @^system dependencies@>
//!
//! @p function init_terminal:boolean; {gets the terminal input started}
//! label exit;
//! begin t_open_in;
//! loop@+begin wake_up_terminal; write(term_out,'**'); update_terminal;
//! @.**@>
//!   if not input_ln(term_in,true) then {this shouldn't happen}
//!     begin write_ln(term_out);
//!     write(term_out,'! End of file on the terminal... why?');
//! @.End of file on the terminal@>
//!     init_terminal:=false; return;
//!     end;
//!   loc:=first;
//!   while (loc<last)and(buffer[loc]=" ") do incr(loc);
//!   if loc<last then
//!     begin init_terminal:=true;
//!     return; {return unless the line was all blank}
//!     end;
//!   write_ln(term_out,'Please type the name of your input file.');
//!   end;
//! exit:end;
//!
