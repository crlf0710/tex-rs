//! @ @<Show equivalent |n|, in region 5@>=
//! begin if n<count_base then print_param(n-int_base)
//! else if  n<del_code_base then
//!   begin print_esc("count"); print_int(n-count_base);
//!   end
//! else  begin print_esc("delcode"); print_int(n-del_code_base);
//!   end;
//! print_char("="); print_int(eqtb[n].int);
//! end
//!
//! @ @<Set variable |c| to the current escape character@>=c:=escape_char
//!
//! @ @<Character |s| is the current new-line character@>=s=new_line_char
//!
//! @ \TeX\ is occasionally supposed to print diagnostic information that
//! goes only into the transcript file, unless |tracing_online| is positive.
//! Here are two routines that adjust the destination of print commands:
//!
//! @p procedure begin_diagnostic; {prepare to do some tracing}
//! begin old_setting:=selector;
//! if (tracing_online<=0)and(selector=term_and_log) then
//!   begin decr(selector);
//!   if history=spotless then history:=warning_issued;
//!   end;
//! end;
//! @#
//! procedure end_diagnostic(@!blank_line:boolean);
//!   {restore proper conditions after tracing}
//! begin print_nl("");
//! if blank_line then print_ln;
//! selector:=old_setting;
//! end;
//!
//! @ Of course we had better declare another global variable, if the previous
//! routines are going to work.
//!
//! @<Glob...@>=
//! @!old_setting:0..max_selector;
//!
