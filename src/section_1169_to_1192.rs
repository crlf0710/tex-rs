//! @ The routine that inserts a |style_node| holds no surprises.
//!
//! @<Put each...@>=
//! primitive("displaystyle",math_style,display_style);
//! @!@:display_style_}{\.{\\displaystyle} primitive@>
//! primitive("textstyle",math_style,text_style);
//! @!@:text_style_}{\.{\\textstyle} primitive@>
//! primitive("scriptstyle",math_style,script_style);
//! @!@:script_style_}{\.{\\scriptstyle} primitive@>
//! primitive("scriptscriptstyle",math_style,script_script_style);
//! @!@:script_script_style_}{\.{\\scriptscriptstyle} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! math_style: print_style(chr_code);
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+math_style: tail_append(new_style(cur_chr));
//! mmode+non_script: begin tail_append(new_glue(zero_glue));
//!   subtype(tail):=cond_math_glue;
//!   end;
//! mmode+math_choice: append_choices;
//!
//! @ The routine that scans the four mlists of a \.{\\mathchoice} is very
//! much like the routine that builds discretionary nodes.
//!
//! @<Declare act...@>=
//! procedure append_choices;
//! begin tail_append(new_choice); incr(save_ptr); saved(-1):=0;
//! push_math(math_choice_group); scan_left_brace;
//! end;
//!
//! @ @<Cases of |handle_right_brace|...@>=
//! math_choice_group: build_choices;
//!
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
//! @ Now at last we're ready to see what happens when a right brace occurs
//! in a math formula. Two special cases are simplified here: Braces are effectively
//! removed when they surround a single Ord without sub/superscripts, or when they
//! surround an accent that is the nucleus of an Ord atom.
//!
//! @<Cases of |handle...@>=
//! math_group: begin unsave; decr(save_ptr);@/
//!   math_type(saved(0)):=sub_mlist; p:=fin_mlist(null); info(saved(0)):=p;
//!   if p<>null then if link(p)=null then
//!    if type(p)=ord_noad then
//!     begin if math_type(subscr(p))=empty then
//!      if math_type(supscr(p))=empty then
//!       begin mem[saved(0)].hh:=mem[nucleus(p)].hh;
//!       free_node(p,noad_size);
//!       end;
//!     end
//!   else if type(p)=accent_noad then if saved(0)=nucleus(tail) then
//!    if type(tail)=ord_noad then @<Replace the tail of the list by |p|@>;
//!   end;
//!
//! @ @<Replace the tail...@>=
//! begin q:=head; while link(q)<>tail do q:=link(q);
//! link(q):=p; free_node(tail,noad_size); tail:=p;
//! end
//!
//! @ We have dealt with all constructions of math mode except `\.{\\left}' and
//! `\.{\\right}', so the picture is completed by the following sections of
//! the program.
//!
//! @<Put each...@>=
//! primitive("left",left_right,left_noad);
//! @!@:left_}{\.{\\left} primitive@>
//! primitive("right",left_right,right_noad);
//! @!@:right_}{\.{\\right} primitive@>
//! text(frozen_right):="right"; eqtb[frozen_right]:=eqtb[cur_val];
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! left_right: if chr_code=left_noad then print_esc("left")
//! else print_esc("right");
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+left_right: math_left_right;
//!
//! @ @<Declare act...@>=
//! procedure math_left_right;
//! var t:small_number; {|left_noad| or |right_noad|}
//! @!p:pointer; {new noad}
//! begin t:=cur_chr;
//! if (t=right_noad)and(cur_group<>math_left_group) then
//!   @<Try to recover from mismatched \.{\\right}@>
//! else  begin p:=new_noad; type(p):=t;
//!   scan_delimiter(delimiter(p),false);
//!   if t=left_noad then
//!     begin push_math(math_left_group); link(head):=p; tail:=p;
//!     end
//!   else  begin p:=fin_mlist(p); unsave; {end of |math_left_group|}
//!     tail_append(new_noad); type(tail):=inner_noad;
//!     math_type(nucleus(tail)):=sub_mlist;
//!     info(nucleus(tail)):=p;
//!     end;
//!   end;
//! end;
//!
//! @ @<Try to recover from mismatch...@>=
//! begin if cur_group=math_shift_group then
//!   begin scan_delimiter(garbage,false);
//!   print_err("Extra "); print_esc("right");
//! @.Extra \\right.@>
//!   help1("I'm ignoring a \right that had no matching \left.");
//!   error;
//!   end
//! else off_save;
//! end
//!
