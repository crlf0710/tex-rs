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
//! ideas here:  For a runaway definition or a runaway balanced text
//! we will insert a right brace; for a
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
