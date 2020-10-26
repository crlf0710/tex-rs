//! @* \[26] Basic scanning subroutines.
//! Let's turn now to some procedures that \TeX\ calls upon frequently to digest
//! certain kinds of patterns in the input. Most of these are quite simple;
//! some are quite elaborate. Almost all of the routines call |get_x_token|,
//! which can cause them to be invoked recursively.
//! @^stomach@>
//! @^recursion@>
//!
//! @ The |scan_left_brace| routine is called when a left brace is supposed to be
//! the next non-blank token. (The term ``left brace'' means, more precisely,
//! a character whose catcode is |left_brace|.) \TeX\ allows \.{\\relax} to
//! appear before the |left_brace|.
//!
//! @p procedure scan_left_brace; {reads a mandatory |left_brace|}
//! begin @<Get the next non-blank non-relax non-call token@>;
//! if cur_cmd<>left_brace then
//!   begin print_err("Missing { inserted");
//! @.Missing \{ inserted@>
//!   help4("A left brace was mandatory here, so I've put one in.")@/
//!     ("You might want to delete and/or insert some corrections")@/
//!     ("so that I will find a matching right brace soon.")@/
//!     ("(If you're confused by all this, try typing `I}' now.)");
//!   back_error; cur_tok:=left_brace_token+"{"; cur_cmd:=left_brace;
//!   cur_chr:="{"; incr(align_state);
//!   end;
//! end;
//!
//! @ @<Get the next non-blank non-relax non-call token@>=
//! repeat get_x_token;
//! until (cur_cmd<>spacer)and(cur_cmd<>relax)
//!
//! @ The |scan_optional_equals| routine looks for an optional `\.=' sign preceded
//! by optional spaces; `\.{\\relax}' is not ignored here.
//!
//! @p procedure scan_optional_equals;
//! begin  @<Get the next non-blank non-call token@>;
//! if cur_tok<>other_token+"=" then back_input;
//! end;
//!
