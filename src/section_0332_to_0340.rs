//! @* \[24] Getting the next token.
//! The heart of \TeX's input mechanism is the |get_next| procedure, which
//! we shall develop in the next few sections of the program. Perhaps we
//! shouldn't actually call it the ``heart,'' however, because it really acts
//! as \TeX's eyes and mouth, reading the source files and gobbling them up.
//! And it also helps \TeX\ to regurgitate stored token lists that are to be
//! processed again.
//! @^eyes and mouth@>
//!
//! The main duty of |get_next| is to input one token and to set |cur_cmd|
//! and |cur_chr| to that token's command code and modifier. Furthermore, if
//! the input token is a control sequence, the |eqtb| location of that control
//! sequence is stored in |cur_cs|; otherwise |cur_cs| is set to zero.
//!
//! Underlying this simple description is a certain amount of complexity
//! because of all the cases that need to be handled.
//! However, the inner loop of |get_next| is reasonably short and fast.
//!
//! When |get_next| is asked to get the next token of a \.{\\read} line,
//! it sets |cur_cmd=cur_chr=cur_cs=0| in the case that no more tokens
//! appear on that line. (There might not be any tokens at all, if the
//! |end_line_char| has |ignore| as its catcode.)
//!
//! @ The value of |par_loc| is the |eqtb| address of `\.{\\par}'. This quantity
//! is needed because a blank line of input is supposed to be exactly equivalent
//! to the appearance of \.{\\par}; we must set |cur_cs:=par_loc|
//! when detecting a blank line.
//!
//! @<Glob...@>=
//! @!par_loc:pointer; {location of `\.{\\par}' in |eqtb|}
//! @!par_token:halfword; {token representing `\.{\\par}'}
//!
//! @ @<Put each...@>=
//! primitive("par",par_end,256); {cf.\ |scan_file_name|}
//! @!@:par_}{\.{\\par} primitive@>
//! par_loc:=cur_val; par_token:=cs_token_flag+par_loc;
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! par_end:print_esc("par");
//!
//! @ Before getting into |get_next|, let's consider the subroutine that
//! is called when an `\.{\\outer}' control sequence has been scanned or
//! when the end of a file has been reached. These two cases are distinguished
//! by |cur_cs|, which is zero at the end of a file.
//!
//! @p procedure check_outer_validity;
//! var p:pointer; {points to inserted token list}
//! @!q:pointer; {auxiliary pointer}
//! begin if scanner_status<>normal then
//!   begin deletions_allowed:=false;
//!   @<Back up an outer control sequence so that it can be reread@>;
//!   if scanner_status>skipping then
//!     @<Tell the user what has run away and try to recover@>
//!   else  begin print_err("Incomplete "); print_cmd_chr(if_test,cur_if);
//! @.Incomplete \\if...@>
//!     print("; all text was ignored after line "); print_int(skip_line);
//!     help3("A forbidden control sequence occurred in skipped text.")@/
//!     ("This kind of error happens when you say `\if...' and forget")@/
//!     ("the matching `\fi'. I've inserted a `\fi'; this might work.");
//!     if cur_cs<>0 then cur_cs:=0
//!     else help_line[2]:=@|
//!       "The file ended while I was skipping conditional text.";
//!     cur_tok:=cs_token_flag+frozen_fi; ins_error;
//!     end;
//!   deletions_allowed:=true;
//!   end;
//! end;
//!
//! @ An outer control sequence that occurs in a \.{\\read} will not be reread,
//! since the error recovery for \.{\\read} is not very powerful.
//!
//! @<Back up an outer control sequence so that it can be reread@>=
//! if cur_cs<>0 then
//!   begin if (state=token_list)or(name<1)or(name>17) then
//!     begin p:=get_avail; info(p):=cs_token_flag+cur_cs;
//!     back_list(p); {prepare to read the control sequence again}
//!     end;
//!   cur_cmd:=spacer; cur_chr:=" "; {replace it by a space}
//!   end
//!
//! @ @<Tell the user what has run away...@>=
//! begin runaway; {print a definition, argument, or preamble}
//! if cur_cs=0 then print_err("File ended")
//! @.File ended while scanning...@>
//! else  begin cur_cs:=0; print_err("Forbidden control sequence found");
//! @.Forbidden control sequence...@>
//!   end;
//! print(" while scanning ");
//! @<Print either `\.{definition}' or `\.{use}' or `\.{preamble}' or `\.{text}',
//!   and insert tokens that should lead to recovery@>;
//! print(" of "); sprint_cs(warning_index);
//! help4("I suspect you have forgotten a `}', causing me")@/
//! ("to read past where you wanted me to stop.")@/
//! ("I'll try to recover; but if the error is serious,")@/
//! ("you'd better type `E' or `X' now and fix your file.");@/
//! error;
//! end
//!
//! @ The recovery procedure can't be fully understood without knowing more
//! about the \TeX\ routines that should be aborted, but we can sketch the
//! ideas here:  For a runaway definition we will insert a right brace; for a
//! runaway preamble, we will insert a special \.{\\cr} token and a right
//! brace; and for a runaway argument, we will set |long_state| to
//! |outer_call| and insert \.{\\par}.
//!
//! @<Print either `\.{definition}' or ...@>=
//! p:=get_avail;
//! case scanner_status of
//! defining:begin print("definition"); info(p):=right_brace_token+"}";
//!   end;
//! matching:begin print("use"); info(p):=par_token; long_state:=outer_call;
//!   end;
//! aligning:begin print("preamble"); info(p):=right_brace_token+"}"; q:=p;
//!   p:=get_avail; link(p):=q; info(p):=cs_token_flag+frozen_cr;
//!   align_state:=-1000000;
//!   end;
//! absorbing:begin print("text"); info(p):=right_brace_token+"}";
//!   end;
//! end; {there are no other cases}
//! ins_list(p)
//!
//! @ We need to mention a procedure here that may be called by |get_next|.
//!
//! @p procedure@?firm_up_the_line; forward;
//!
