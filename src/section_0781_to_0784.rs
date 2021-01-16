//! @ @<Cases of |print_cmd_chr|...@>=
//! tab_mark: if chr_code=span_code then print_esc("span")
//!   else chr_cmd("alignment tab character ");
//! car_ret: if chr_code=cr_code then print_esc("cr")
//!   else print_esc("crcr");
//!
//! @ The preamble is copied directly, except that \.{\\tabskip} causes a change
//! to the tabskip glue, thereby possibly expanding macros that immediately
//! follow it. An appearance of \.{\\span} also causes such an expansion.
//!
//! Note that if the preamble contains `\.{\\global\\tabskip}', the `\.{\\global}'
//! token survives in the preamble and the `\.{\\tabskip}' defines new
//! tabskip glue (locally).
//!
//! @<Declare the procedure called |get_preamble_token|@>=
//! procedure get_preamble_token;
//! label restart;
//! begin restart: get_token;
//! while (cur_chr=span_code)and(cur_cmd=tab_mark) do
//!   begin get_token; {this token will be expanded once}
//!   if cur_cmd>max_command then
//!     begin expand; get_token;
//!     end;
//!   end;
//! if cur_cmd=endv then
//!   fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//! if (cur_cmd=assign_glue)and(cur_chr=glue_base+tab_skip_code) then
//!   begin scan_optional_equals; scan_glue(glue_val);
//!   if global_defs>0 then geq_define(glue_base+tab_skip_code,glue_ref,cur_val)
//!   else eq_define(glue_base+tab_skip_code,glue_ref,cur_val);
//!   goto restart;
//!   end;
//! end;
//!
//! @ Spaces are eliminated from the beginning of a template.
//!
//! @<Scan the template \<u_j>...@>=
//! p:=hold_head; link(p):=null;
//! loop@+  begin get_preamble_token;
//!   if cur_cmd=mac_param then goto done1;
//!   if (cur_cmd<=car_ret)and(cur_cmd>=tab_mark)and(align_state=-1000000) then
//!    if (p=hold_head)and(cur_loop=null)and(cur_cmd=tab_mark)
//!     then cur_loop:=cur_align
//!    else  begin print_err("Missing # inserted in alignment preamble");
//! @.Missing \# inserted...@>
//!     help3("There should be exactly one # between &'s, when an")@/
//!     ("\halign or \valign is being set up. In this case you had")@/
//!     ("none, so I've put one in; maybe that will work.");
//!     back_error; goto done1;
//!     end
//!   else if (cur_cmd<>spacer)or(p<>hold_head) then
//!     begin link(p):=get_avail; p:=link(p); info(p):=cur_tok;
//!     end;
//!   end;
//! done1:
//!
//! @ @<Scan the template \<v_j>...@>=
//! p:=hold_head; link(p):=null;
//! loop@+  begin continue: get_preamble_token;
//!   if (cur_cmd<=car_ret)and(cur_cmd>=tab_mark)and(align_state=-1000000) then
//!     goto done2;
//!   if cur_cmd=mac_param then
//!     begin print_err("Only one # is allowed per tab");
//! @.Only one \# is allowed...@>
//!     help3("There should be exactly one # between &'s, when an")@/
//!     ("\halign or \valign is being set up. In this case you had")@/
//!     ("more than one, so I'm ignoring all but the first.");
//!     error; goto continue;
//!     end;
//!   link(p):=get_avail; p:=link(p); info(p):=cur_tok;
//!   end;
//! done2: link(p):=get_avail; p:=link(p);
//! info(p):=end_template_token {put \.{\\endtemplate} at the end}
//!
