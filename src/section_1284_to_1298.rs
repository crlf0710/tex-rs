//! @ The |error| routine calls on |give_err_help| if help is requested from
//! the |err_help| parameter.
//!
//! @p procedure give_err_help;
//! begin token_show(err_help);
//! end;
//!
//! @ The \.{\\uppercase} and \.{\\lowercase} commands are implemented by
//! building a token list and then changing the cases of the letters in it.
//!
//! @<Cases of |main_control| that don't...@>=
//! any_mode(case_shift):shift_case;
//!
//! @ @<Put each...@>=
//! primitive("lowercase",case_shift,lc_code_base);
//! @!@:lowercase_}{\.{\\lowercase} primitive@>
//! primitive("uppercase",case_shift,uc_code_base);
//! @!@:uppercase_}{\.{\\uppercase} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! case_shift:if chr_code=lc_code_base then print_esc("lowercase")
//!   else print_esc("uppercase");
//!
//! @ @<Declare act...@>=
//! procedure shift_case;
//! var b:pointer; {|lc_code_base| or |uc_code_base|}
//! @!p:pointer; {runs through the token list}
//! @!t:halfword; {token}
//! @!c:eight_bits; {character code}
//! begin b:=cur_chr; p:=scan_toks(false,false); p:=link(def_ref);
//! while p<>null do
//!   begin @<Change the case of the token in |p|, if a change is appropriate@>;
//!   p:=link(p);
//!   end;
//! back_list(link(def_ref)); free_avail(def_ref); {omit reference count}
//! end;
//!
//! @ When the case of a |chr_code| changes, we don't change the |cmd|.
//! We also change active characters, using the fact that
//! |cs_token_flag+active_base| is a multiple of~256.
//! @^data structure assumptions@>
//!
//! @<Change the case of the token in |p|, if a change is appropriate@>=
//! t:=info(p);
//! if t<cs_token_flag+single_base then
//!   begin c:=t mod 256;
//!   if equiv(b+c)<>0 then info(p):=t-c+equiv(b+c);
//!   end
//!
//! @ We come finally to the last pieces missing from |main_control|, namely the
//! `\.{\\show}' commands that are useful when debugging.
//!
//! @<Cases of |main_control| that don't...@>=
//! any_mode(xray): show_whatever;
//!
//! @ @d show_code=0 { \.{\\show} }
//! @d show_box_code=1 { \.{\\showbox} }
//! @d show_the_code=2 { \.{\\showthe} }
//! @d show_lists=3 { \.{\\showlists} }
//!
//! @<Put each...@>=
//! primitive("show",xray,show_code);
//! @!@:show_}{\.{\\show} primitive@>
//! primitive("showbox",xray,show_box_code);
//! @!@:show_box_}{\.{\\showbox} primitive@>
//! primitive("showthe",xray,show_the_code);
//! @!@:show_the_}{\.{\\showthe} primitive@>
//! primitive("showlists",xray,show_lists);
//! @!@:show_lists_}{\.{\\showlists} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! xray: case chr_code of
//!   show_box_code:print_esc("showbox");
//!   show_the_code:print_esc("showthe");
//!   show_lists:print_esc("showlists");
//!   othercases print_esc("show")
//!   endcases;
//!
//! @ @<Declare act...@>=
//! procedure show_whatever;
//! label common_ending;
//! var p:pointer; {tail of a token list to show}
//! begin case cur_chr of
//! show_lists: begin begin_diagnostic; show_activities;
//!   end;
//! show_box_code: @<Show the current contents of a box@>;
//! show_code: @<Show the current meaning of a token, then |goto common_ending|@>;
//! othercases @<Show the current value of some parameter or register,
//!   then |goto common_ending|@>
//! endcases;@/
//! @<Complete a potentially long \.{\\show} command@>;
//! common_ending: if interaction<error_stop_mode then
//!   begin help0; decr(error_count);
//!   end
//! else if tracing_online>0 then
//!   begin@t@>@;@/
//!   help3("This isn't an error message; I'm just \showing something.")@/
//!   ("Type `I\show...' to show more (e.g., \show\cs,")@/
//!   ("\showthe\count10, \showbox255, \showlists).");
//!   end
//! else  begin@t@>@;@/
//!   help5("This isn't an error message; I'm just \showing something.")@/
//!   ("Type `I\show...' to show more (e.g., \show\cs,")@/
//!   ("\showthe\count10, \showbox255, \showlists).")@/
//!   ("And type `I\tracingonline=1\show...' to show boxes and")@/
//!   ("lists on your terminal as well as in the transcript file.");
//!   end;
//! error;
//! end;
//!
//! @ @<Show the current meaning of a token...@>=
//! begin get_token;
//! if interaction=error_stop_mode then wake_up_terminal;
//! print_nl("> ");
//! if cur_cs<>0 then
//!   begin sprint_cs(cur_cs); print_char("=");
//!   end;
//! print_meaning; goto common_ending;
//! end
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! undefined_cs: print("undefined");
//! call: print("macro");
//! long_call: print_esc("long macro");
//! outer_call: print_esc("outer macro");
//! long_outer_call: begin print_esc("long"); print_esc("outer macro");
//!   end;
//! end_template: print_esc("outer endtemplate");
//!
//! @ @<Show the current contents of a box@>=
//! begin scan_eight_bit_int; begin_diagnostic;
//! print_nl("> \box"); print_int(cur_val); print_char("=");
//! if box(cur_val)=null then print("void")
//! else show_box(box(cur_val));
//! end
//!
//! @ @<Show the current value of some parameter...@>=
//! begin p:=the_toks;
//! if interaction=error_stop_mode then wake_up_terminal;
//! print_nl("> "); token_show(temp_head);
//! flush_list(link(temp_head)); goto common_ending;
//! endz
//!
//! @ @<Complete a potentially long \.{\\show} command@>=
//! end_diagnostic(true); print_err("OK");
//! @.OK@>
//! if selector=term_and_log then if tracing_online<=0 then
//!   begin selector:=term_only; print(" (see the transcript file)");
//!   selector:=term_and_log;
//!   end
//!
