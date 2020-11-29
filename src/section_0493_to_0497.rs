//! @ When we skip conditional text, we keep track of the line number
//! where skipping began, for use in error messages.
//!
//! @<Glob...@>=
//! @!skip_line:integer; {skipping began here}
//!
//! @ Here is a procedure that ignores text until coming to an \.{\\or},
//! \.{\\else}, or \.{\\fi} at level zero of $\.{\\if}\ldots\.{\\fi}$
//! nesting. After it has acted, |cur_chr| will indicate the token that
//! was found, but |cur_tok| will not be set (because this makes the
//! procedure run faster).
//!
//! @p procedure pass_text;
//! label done;
//! var l:integer; {level of $\.{\\if}\ldots\.{\\fi}$ nesting}
//! @!save_scanner_status:small_number; {|scanner_status| upon entry}
//! begin save_scanner_status:=scanner_status; scanner_status:=skipping; l:=0;
//! skip_line:=line;
//! loop@+  begin get_next;
//!   if cur_cmd=fi_or_else then
//!     begin if l=0 then goto done;
//!     if cur_chr=fi_code then decr(l);
//!     end
//!   else if cur_cmd=if_test then incr(l);
//!   end;
//! done: scanner_status:=save_scanner_status;
//! end;
//!
//! @ When we begin to process a new \.{\\if}, we set |if_limit:=if_code|; then
//! if\/ \.{\\or} or \.{\\else} or \.{\\fi} occurs before the current \.{\\if}
//! condition has been evaluated, \.{\\relax} will be inserted.
//! For example, a sequence of commands like `\.{\\ifvoid1\\else...\\fi}'
//! would otherwise require something after the `\.1'.
//!
//! @<Push the condition stack@>=
//! begin p:=get_node(if_node_size); link(p):=cond_ptr; type(p):=if_limit;
//! subtype(p):=cur_if; if_line_field(p):=if_line;
//! cond_ptr:=p; cur_if:=cur_chr; if_limit:=if_code; if_line:=line;
//! end
//!
//! @ @<Pop the condition stack@>=
//! begin p:=cond_ptr; if_line:=if_line_field(p);
//! cur_if:=subtype(p); if_limit:=type(p); cond_ptr:=link(p);
//! free_node(p,if_node_size);
//! end
//!
//! @ Here's a procedure that changes the |if_limit| code corresponding to
//! a given value of |cond_ptr|.
//!
//! @p procedure change_if_limit(@!l:small_number;@!p:pointer);
//! label exit;
//! var q:pointer;
//! begin if p=cond_ptr then if_limit:=l {that's the easy case}
//! else  begin q:=cond_ptr;
//!   loop@+  begin if q=null then confusion("if");
//! @:this can't happen if}{\quad if@>
//!     if link(q)=p then
//!       begin type(q):=l; return;
//!       end;
//!     q:=link(q);
//!     end;
//!   end;
//! exit:end;
//!
