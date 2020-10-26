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
