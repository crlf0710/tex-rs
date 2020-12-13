//! @ All glue parameters and registers are initially `\.{0pt plus0pt minus0pt}'.
//!
//! @<Initialize table entries...@>=
//! equiv(glue_base):=zero_glue; eq_level(glue_base):=level_one;
//! eq_type(glue_base):=glue_ref;
//! for k:=glue_base+1 to local_base-1 do eqtb[k]:=eqtb[glue_base];
//! glue_ref_count(zero_glue):=glue_ref_count(zero_glue)+local_base-glue_base;
//!
//! @ @<Show equivalent |n|, in region 3@>=
//! if n<skip_base then
//!   begin print_skip_param(n-glue_base); print_char("=");
//!   if n<glue_base+thin_mu_skip_code then print_spec(equiv(n),"pt")
//!   else print_spec(equiv(n),"mu");
//!   end
//! else if n<mu_skip_base then
//!   begin print_esc("skip"); print_int(n-skip_base); print_char("=");
//!   print_spec(equiv(n),"pt");
//!   end
//! else  begin print_esc("muskip"); print_int(n-mu_skip_base); print_char("=");
//!   print_spec(equiv(n),"mu");
//!   end
//!
