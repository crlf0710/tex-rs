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
//! @ An operation like `\.{\\over}' causes the current mlist to go into a
//! state of suspended animation: |incompleat_noad| points to a |fraction_noad|
//! that contains the mlist-so-far as its numerator, while the denominator
//! is yet to come. Finally when the mlist is finished, the denominator will
//! go into the incompleat fraction noad, and that noad will become the
//! whole formula, unless it is surrounded by `\.{\\left}' and `\.{\\right}'
//! delimiters.
//!
//! @d above_code=0 { `\.{\\above}' }
//! @d over_code=1 { `\.{\\over}' }
//! @d atop_code=2 { `\.{\\atop}' }
//! @d delimited_code=3 { `\.{\\abovewithdelims}', etc.}
//!
//! @<Put each...@>=
//! primitive("above",above,above_code);@/
//! @!@:above_}{\.{\\above} primitive@>
//! primitive("over",above,over_code);@/
//! @!@:over_}{\.{\\over} primitive@>
//! primitive("atop",above,atop_code);@/
//! @!@:atop_}{\.{\\atop} primitive@>
//! primitive("abovewithdelims",above,delimited_code+above_code);@/
//! @!@:above_with_delims_}{\.{\\abovewithdelims} primitive@>
//! primitive("overwithdelims",above,delimited_code+over_code);@/
//! @!@:over_with_delims_}{\.{\\overwithdelims} primitive@>
//! primitive("atopwithdelims",above,delimited_code+atop_code);
//! @!@:atop_with_delims_}{\.{\\atopwithdelims} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! above: case chr_code of
//!   over_code:print_esc("over");
//!   atop_code:print_esc("atop");
//!   delimited_code+above_code:print_esc("abovewithdelims");
//!   delimited_code+over_code:print_esc("overwithdelims");
//!   delimited_code+atop_code:print_esc("atopwithdelims");
//!   othercases print_esc("above")
//!   endcases;
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+above: math_fraction;
//!
//! @ @<Declare act...@>=
//! procedure math_fraction;
//! var c:small_number; {the type of generalized fraction we are scanning}
//! begin c:=cur_chr;
//! if incompleat_noad<>null then
//!   @<Ignore the fraction operation and complain about this ambiguous case@>
//! else  begin incompleat_noad:=get_node(fraction_noad_size);
//!   type(incompleat_noad):=fraction_noad;
//!   subtype(incompleat_noad):=normal;
//!   math_type(numerator(incompleat_noad)):=sub_mlist;
//!   info(numerator(incompleat_noad)):=link(head);
//!   mem[denominator(incompleat_noad)].hh:=empty_field;
//!   mem[left_delimiter(incompleat_noad)].qqqq:=null_delimiter;
//!   mem[right_delimiter(incompleat_noad)].qqqq:=null_delimiter;@/
//!   link(head):=null; tail:=head;
//!   @<Use code |c| to distinguish between generalized fractions@>;
//!   end;
//! end;
//!
//! @ @<Use code |c|...@>=
//! if c>=delimited_code then
//!   begin scan_delimiter(left_delimiter(incompleat_noad),false);
//!   scan_delimiter(right_delimiter(incompleat_noad),false);
//!   end;
//! case c mod delimited_code of
//! above_code: begin scan_normal_dimen;
//!   thickness(incompleat_noad):=cur_val;
//!   end;
//! over_code: thickness(incompleat_noad):=default_code;
//! atop_code: thickness(incompleat_noad):=0;
//! end {there are no other cases}
//!
//! @ @<Ignore the fraction...@>=
//! begin if c>=delimited_code then
//!   begin scan_delimiter(garbage,false); scan_delimiter(garbage,false);
//!   end;
//! if c mod delimited_code=above_code then scan_normal_dimen;
//! print_err("Ambiguous; you need another { and }");
//! @.Ambiguous...@>
//! help3("I'm ignoring this fraction specification, since I don't")@/
//!   ("know whether a construction like `x \over y \over z'")@/
//!   ("means `{x \over y} \over z' or `x \over {y \over z}'.");
//! error;
//! end
//!
//! @ At the end of a math formula or subformula, the |fin_mlist| routine is
//! called upon to return a pointer to the newly completed mlist, and to
//! pop the nest back to the enclosing semantic level. The parameter to
//! |fin_mlist|, if not null, points to a |right_noad| that ends the
//! current mlist; this |right_noad| has not yet been appended.
//!
//! @<Declare the function called |fin_mlist|@>=
//! function fin_mlist(@!p:pointer):pointer;
//! var q:pointer; {the mlist to return}
//! begin if incompleat_noad<>null then @<Compleat the incompleat noad@>
//! else  begin link(tail):=p; q:=link(head);
//!   end;
//! pop_nest; fin_mlist:=q;
//! end;
//!
//! @ @<Compleat...@>=
//! begin math_type(denominator(incompleat_noad)):=sub_mlist;
//! info(denominator(incompleat_noad)):=link(head);
//! if p=null then q:=incompleat_noad
//! else  begin q:=info(numerator(incompleat_noad));
//!   if type(q)<>left_noad then confusion("right");
//! @:this can't happen right}{\quad right@>
//!   info(numerator(incompleat_noad)):=link(q);
//!   link(q):=incompleat_noad; link(incompleat_noad):=p;
//!   end;
//! end
//!
