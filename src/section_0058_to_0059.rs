//! @ The |print_char| procedure sends one character to the desired destination,
//! using the |xchr| array to map it into an external character compatible with
//! |input_ln|. All printing comes through |print_ln| or |print_char|.
//!
//! @<Basic printing...@>=
//! procedure print_char(@!s:ASCII_code); {prints a single character}
//! label exit;
//! begin if @<Character |s| is the current new-line character@> then
//!  if selector<pseudo then
//!   begin print_ln; return;
//!   end;
//! case selector of
//! term_and_log: begin wterm(xchr[s]); wlog(xchr[s]);
//!   incr(term_offset); incr(file_offset);
//!   if term_offset=max_print_line then
//!     begin wterm_cr; term_offset:=0;
//!     end;
//!   if file_offset=max_print_line then
//!     begin wlog_cr; file_offset:=0;
//!     end;
//!   end;
//! log_only: begin wlog(xchr[s]); incr(file_offset);
//!   if file_offset=max_print_line then print_ln;
//!   end;
//! term_only: begin wterm(xchr[s]); incr(term_offset);
//!   if term_offset=max_print_line then print_ln;
//!   end;
//! no_print: do_nothing;
//! pseudo: if tally<trick_count then trick_buf[tally mod error_line]:=s;
//! new_string: begin if pool_ptr<pool_size then append_char(s);
//!   end; {we drop characters if the string space is full}
//! othercases write(write_file[selector],xchr[s])
//! endcases;@/
//! incr(tally);
//! exit:end;
//!
//! @ An entire string is output by calling |print|. Note that if we are outputting
//! the single standard ASCII character \.c, we could call |print("c")|, since
//! |"c"=99| is the number of a single-character string, as explained above. But
//! |print_char("c")| is quicker, so \TeX\ goes directly to the |print_char|
//! routine when it knows that this is safe. (The present implementation
//! assumes that it is always safe to print a visible ASCII character.)
//! @^system dependencies@>
//!
//! @<Basic print...@>=
//! procedure print(@!s:integer); {prints string |s|}
//! label exit;
//! var j:pool_pointer; {current character code position}
//! @!nl:integer; {new-line character to restore}
//! begin if s>=str_ptr then s:="???" {this can't happen}
//! @.???@>
//! else if s<256 then
//!   if s<0 then s:="???" {can't happen}
//!   else begin if selector>pseudo then
//!       begin print_char(s); return; {internal strings are not expanded}
//!       end;
//!     if (@<Character |s| is the current new-line character@>) then
//!       if selector<pseudo then
//!         begin print_ln; return;
//!         end;
//!     nl:=new_line_char; new_line_char:=-1;
//!       {temporarily disable new-line character}
//!     j:=str_start[s];
//!     while j<str_start[s+1] do
//!       begin print_char(so(str_pool[j])); incr(j);
//!       end;
//!     new_line_char:=nl; return;
//!     end;
//! j:=str_start[s];
//! while j<str_start[s+1] do
//!   begin print_char(so(str_pool[j])); incr(j);
//!   end;
//! exit:end;
//!
