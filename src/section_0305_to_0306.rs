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
