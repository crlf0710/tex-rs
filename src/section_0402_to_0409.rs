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
//! @ @<Get the next non-blank non-call token@>=
//! repeat get_x_token;
//! until cur_cmd<>spacer
//!
//! @ In case you are getting bored, here is a slightly less trivial routine:
//! Given a string of lowercase letters, like `\.{pt}' or `\.{plus}' or
//! `\.{width}', the |scan_keyword| routine checks to see whether the next
//! tokens of input match this string. The match must be exact, except that
//! uppercase letters will match their lowercase counterparts; uppercase
//! equivalents are determined by subtracting |"a"-"A"|, rather than using the
//! |uc_code| table, since \TeX\ uses this routine only for its own limited
//! set of keywords.
//!
//! If a match is found, the characters are effectively removed from the input
//! and |true| is returned. Otherwise |false| is returned, and the input
//! is left essentially unchanged (except for the fact that some macros
//! may have been expanded, etc.).
//! @^inner loop@>
//!
//! @p function scan_keyword(@!s:str_number):boolean; {look for a given string}
//! label exit;
//! var p:pointer; {tail of the backup list}
//! @!q:pointer; {new node being added to the token list via |store_new_token|}
//! @!k:pool_pointer; {index into |str_pool|}
//! begin p:=backup_head; link(p):=null; k:=str_start[s];
//! while k<str_start[s+1] do
//!   begin get_x_token; {recursion is possible here}
//! @^recursion@>
//!   if (cur_cs=0)and@|
//!    ((cur_chr=so(str_pool[k]))or(cur_chr=so(str_pool[k])-"a"+"A")) then
//!     begin store_new_token(cur_tok); incr(k);
//!     end
//!   else if (cur_cmd<>spacer)or(p<>backup_head) then
//!     begin back_input;
//!     if p<>backup_head then back_list(link(backup_head));
//!     scan_keyword:=false; return;
//!     end;
//!   end;
//! flush_list(link(backup_head)); scan_keyword:=true;
//! exit:end;
//!
//! @ Here is a procedure that sounds an alarm when mu and non-mu units
//! are being switched.
//!
//! @p procedure mu_error;
//! begin print_err("Incompatible glue units");
//! @.Incompatible glue units@>
//! help1("I'm going to assume that 1mu=1pt when they're mixed.");
//! error;
//! end;
//!
//! @ The next routine `|scan_something_internal|' is used to fetch internal
//! numeric quantities like `\.{\\hsize}', and also to handle the `\.{\\the}'
//! when expanding constructions like `\.{\\the\\toks0}' and
//! `\.{\\the\\baselineskip}'. Soon we will be considering the |scan_int|
//! procedure, which calls |scan_something_internal|; on the other hand,
//! |scan_something_internal| also calls |scan_int|, for constructions like
//! `\.{\\catcode\`\\\$}' or `\.{\\fontdimen} \.3 \.{\\ff}'. So we
//! have to declare |scan_int| as a |forward| procedure. A few other
//! procedures are also declared at this point.
//!
//! @p procedure@?scan_int; forward; {scans an integer value}
//! @t\4\4@>@<Declare procedures that scan restricted classes of integers@>@;
//! @t\4\4@>@<Declare procedures that scan font-related stuff@>
//!
