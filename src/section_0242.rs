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
