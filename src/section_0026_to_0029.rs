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
