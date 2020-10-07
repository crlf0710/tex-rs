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
