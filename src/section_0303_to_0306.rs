//! @ Let's look more closely now at the control variables
//! (|state|,~|index|,~|start|,~|loc|,~|limit|,~|name|),
//! assuming that \TeX\ is reading a line of characters that have been input
//! from some file or from the user's terminal. There is an array called
//! |buffer| that acts as a stack of all lines of characters that are
//! currently being read from files, including all lines on subsidiary
//! levels of the input stack that are not yet completed. \TeX\ will return to
//! the other lines when it is finished with the present input file.
//!
//! (Incidentally, on a machine with byte-oriented addressing, it might be
//! appropriate to combine |buffer| with the |str_pool| array,
//! letting the buffer entries grow downward from the top of the string pool
//! and checking that these two tables don't bump into each other.)
//!
//! The line we are currently working on begins in position |start| of the
//! buffer; the next character we are about to read is |buffer[loc]|; and
//! |limit| is the location of the last character present.  If |loc>limit|,
//! the line has been completely read. Usually |buffer[limit]| is the
//! |end_line_char|, denoting the end of a line, but this is not
//! true if the current line is an insertion that was entered on the user's
//! terminal in response to an error message.
//!
//! The |name| variable is a string number that designates the name of
//! the current file, if we are reading a text file. It is zero if we
//! are reading from the terminal; it is |n+1| if we are reading from
//! input stream |n|, where |0<=n<=16|. (Input stream 16 stands for
//! an invalid stream number; in such cases the input is actually from
//! the terminal, under control of the procedure |read_toks|.)
//!
//! The |state| variable has one of three values, when we are scanning such
//! files:
//! $$\baselineskip 15pt\vbox{\halign{#\hfil\cr
//! 1) |state=mid_line| is the normal state.\cr
//! 2) |state=skip_blanks| is like |mid_line|, but blanks are ignored.\cr
//! 3) |state=new_line| is the state at the beginning of a line.\cr}}$$
//! These state values are assigned numeric codes so that if we add the state
//! code to the next character's command code, we get distinct values. For
//! example, `|mid_line+spacer|' stands for the case that a blank
//! space character occurs in the middle of a line when it is not being
//! ignored; after this case is processed, the next value of |state| will
//! be |skip_blanks|.
//!
//! @d mid_line=1 {|state| code when scanning a line of characters}
//! @d skip_blanks=2+max_char_code {|state| code when ignoring blanks}
//! @d new_line=3+max_char_code+max_char_code {|state| code at start of line}
//!
//! @ Additional information about the current line is available via the
//! |index| variable, which counts how many lines of characters are present
//! in the buffer below the current level. We have |index=0| when reading
//! from the terminal and prompting the user for each line; then if the user types,
//! e.g., `\.{\\input paper}', we will have |index=1| while reading
//! the file \.{paper.tex}. However, it does not follow that |index| is the
//! same as the input stack pointer, since many of the levels on the input
//! stack may come from token lists. For example, the instruction `\.{\\input
//! paper}' might occur in a token list.
//!
//! The global variable |in_open| is equal to the |index|
//! value of the highest non-token-list level. Thus, the number of partially read
//! lines in the buffer is |in_open+1|, and we have |in_open=index|
//! when we are not reading a token list.
//!
//! If we are not currently reading from the terminal, or from an input
//! stream, we are reading from the file variable |input_file[index]|. We use
//! the notation |terminal_input| as a convenient abbreviation for |name=0|,
//! and |cur_file| as an abbreviation for |input_file[index]|.
//!
//! The global variable |line| contains the line number in the topmost
//! open file, for use in error messages. If we are not reading from
//! the terminal, |line_stack[index]| holds the line number for the
//! enclosing level, so that |line| can be restored when the current
//! file has been read. Line numbers should never be negative, since the
//! negative of the current line number is used to identify the user's output
//! routine in the |mode_line| field of the semantic nest entries.
//!
//! If more information about the input state is needed, it can be
//! included in small arrays like those shown here. For example,
//! the current page or segment number in the input file might be
//! put into a variable |@!page|, maintained for enclosing levels in
//! `\ignorespaces|@!page_stack:array[1..max_in_open] of integer|\unskip'
//! by analogy with |line_stack|.
//! @^system dependencies@>
//!
//! @d terminal_input==(name=0) {are we reading from the terminal?}
//! @d cur_file==input_file[index] {the current |alpha_file| variable}
//!
//! @<Glob...@>=
//! @!in_open : 0..max_in_open; {the number of lines in the buffer, less one}
//! @!open_parens : 0..max_in_open; {the number of open text files}
//! @!input_file : array[1..max_in_open] of alpha_file;
//! @!line : integer; {current line number in the current source file}
//! @!line_stack : array[1..max_in_open] of integer;
//!
//! @ Users of \TeX\ sometimes forget to balance left and right braces properly,
//! and one of the ways \TeX\ tries to spot such errors is by considering an
//! input file as broken into subfiles by control sequences that
//! are declared to be \.{\\outer}.
//!
//! A variable called |scanner_status| tells \TeX\ whether or not to complain
//! when a subfile ends. This variable has six possible values:
//!
//! \yskip\hang|normal|, means that a subfile can safely end here without incident.
//!
//! \yskip\hang|skipping|, means that a subfile can safely end here, but not a file,
//! because we're reading past some conditional text that was not selected.
//!
//! \yskip\hang|defining|, means that a subfile shouldn't end now because a
//! macro is being defined.
//!
//! \yskip\hang|matching|, means that a subfile shouldn't end now because a
//! macro is being used and we are searching for the end of its arguments.
//!
//! \yskip\hang|aligning|, means that a subfile shouldn't end now because we are
//! not finished with the preamble of an \.{\\halign} or \.{\\valign}.
//!
//! \yskip\hang|absorbing|, means that a subfile shouldn't end now because we are
//! reading a balanced token list for \.{\\message}, \.{\\write}, etc.
//!
//! \yskip\noindent
//! If the |scanner_status| is not |normal|, the variable |warning_index| points
//! to the |eqtb| location for the relevant control sequence name to print
//! in an error message.
//!
//! @d skipping=1 {|scanner_status| when passing conditional text}
//! @d defining=2 {|scanner_status| when reading a macro definition}
//! @d matching=3 {|scanner_status| when reading macro arguments}
//! @d aligning=4 {|scanner_status| when reading an alignment preamble}
//! @d absorbing=5 {|scanner_status| when reading a balanced text}
//!
//! @<Glob...@>=
//! @!scanner_status : normal..absorbing; {can a subfile end now?}
//! @!warning_index : pointer; {identifier relevant to non-|normal| scanner status}
//! @!def_ref : pointer; {reference count of token list being defined}
//!
//! @ Here is a procedure that uses |scanner_status| to print a warning message
//! when a subfile has ended, and at certain other crucial times:
//!
//! @<Declare the procedure called |runaway|@>=
//! procedure runaway;
//! var p:pointer; {head of runaway list}
//! begin if scanner_status>skipping then
//!   begin print_nl("Runaway ");
//! @.Runaway...@>
//!   case scanner_status of
//!   defining: begin print("definition"); p:=def_ref;
//!     end;
//!   matching: begin print("argument"); p:=temp_head;
//!     end;
//!   aligning: begin print("preamble"); p:=hold_head;
//!     end;
//!   absorbing: begin print("text"); p:=def_ref;
//!     end;
//!   end; {there are no other cases}
//!   print_char("?");print_ln; show_token_list(link(p),null,error_line-10);
//!   end;
//! end;
//!
