//! @ @<Declare act...@>=
//! @t\4@>@<Declare the function called |fin_mlist|@>@t@>@;@/
//! procedure build_choices;
//! label exit;
//! var p:pointer; {the current mlist}
//! begin unsave; p:=fin_mlist(null);
//! case saved(-1) of
//! 0:display_mlist(tail):=p;
//! 1:text_mlist(tail):=p;
//! 2:script_mlist(tail):=p;
//! 3:begin script_script_mlist(tail):=p; decr(save_ptr); return;
//!   end;
//! end; {there are no other cases}
//! incr(saved(-1)); push_math(math_choice_group); scan_left_brace;
//! exit:end;
//!
//! @ Subscripts and superscripts are attached to the previous nucleus by the
//! @^superscripts@>@^subscripts@>
//! action procedure called |sub_sup|. We use the facts that |sub_mark=sup_mark+1|
//! and |subscr(p)=supscr(p)+1|.
//!
//! @<Cases of |main_control| that build...@>=
//! mmode+sub_mark,mmode+sup_mark: sub_sup;
//!
//! @ @<Declare act...@>=
//! procedure sub_sup;
//! var t:small_number; {type of previous sub/superscript}
//! @!p:pointer; {field to be filled by |scan_math|}
//! begin t:=empty; p:=null;
//! if tail<>head then if scripts_allowed(tail) then
//!   begin p:=supscr(tail)+cur_cmd-sup_mark; {|supscr| or |subscr|}
//!   t:=math_type(p);
//!   end;
//! if (p=null)or(t<>empty) then @<Insert a dummy noad to be sub/superscripted@>;
//! scan_math(p);
//! end;
//!
//! @ @<Insert a dummy...@>=
//! begin tail_append(new_noad);
//! p:=supscr(tail)+cur_cmd-sup_mark; {|supscr| or |subscr|}
//! if t<>empty then
//!   begin if cur_cmd=sup_mark then
//!     begin print_err("Double superscript");
//! @.Double superscript@>
//!     help1("I treat `x^1^2' essentially like `x^1{}^2'.");
//!     end
//!   else  begin print_err("Double subscript");
//! @.Double subscript@>
//!     help1("I treat `x_1_2' essentially like `x_1{}_2'.");
//!     end;
//!   error;
//!   end;
//! end
//!
