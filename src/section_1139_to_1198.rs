//! @ @<Go into ordinary math mode@>=
//! begin push_math(math_shift_group); eq_word_define(int_base+cur_fam_code,-1);
//! if every_math<>null then begin_token_list(every_math,every_math_text);
//! end
//!
//! @ We get into ordinary math mode from display math mode when `\.{\\eqno}' or
//! `\.{\\leqno}' appears. In such cases |cur_chr| will be 0 or~1, respectively;
//! the value of |cur_chr| is placed onto |save_stack| for safe keeping.
//!
//! @<Cases of |main_control| that build...@>=
//! mmode+eq_no: if privileged then
//!   if cur_group=math_shift_group then start_eq_no
//!   else off_save;
//!
//! @ @<Put each...@>=
//! primitive("eqno",eq_no,0);
//! @!@:eq_no_}{\.{\\eqno} primitive@>
//! primitive("leqno",eq_no,1);
//! @!@:leq_no_}{\.{\\leqno} primitive@>
//!
//! @ When \TeX\ is in display math mode, |cur_group=math_shift_group|,
//! so it is not necessary for the |start_eq_no| procedure to test for
//! this condition.
//!
//! @<Declare act...@>=
//! procedure start_eq_no;
//! begin saved(0):=cur_chr; incr(save_ptr);
//! @<Go into ordinary math mode@>;
//! end;
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! eq_no:if chr_code=1 then print_esc("leqno")@+else print_esc("eqno");
//!
//! @ @<Forbidden...@>=non_math(eq_no),
//!
//! @ When we enter display math mode, we need to call |line_break| to
//! process the partial paragraph that has just been interrupted by the
//! display. Then we can set the proper values of |display_width| and
//! |display_indent| and |pre_display_size|.
//!
//! @<Go into display math mode@>=
//! begin if head=tail then {`\.{\\noindent\$\$}' or `\.{\$\${ }\$\$}'}
//!   begin pop_nest; w:=-max_dimen;
//!   end
//! else  begin line_break(display_widow_penalty);@/
//!   @<Calculate the natural width, |w|, by which the characters of the
//!     final line extend to the right of the reference point,
//!     plus two ems; or set |w:=max_dimen| if the non-blank information
//!     on that line is affected by stretching or shrinking@>;
//!   end;
//! {now we are in vertical mode, working on the list that will contain the display}
//! @<Calculate the length, |l|, and the shift amount, |s|, of the display lines@>;
//! push_math(math_shift_group); mode:=mmode;
//! eq_word_define(int_base+cur_fam_code,-1);@/
//! eq_word_define(dimen_base+pre_display_size_code,w);
//! eq_word_define(dimen_base+display_width_code,l);
//! eq_word_define(dimen_base+display_indent_code,s);
//! if every_display<>null then begin_token_list(every_display,every_display_text);
//! if nest_ptr=1 then build_page;
//! end
//!
//! @ @<Calculate the natural width, |w|, by which...@>=
//! v:=shift_amount(just_box)+2*quad(cur_font); w:=-max_dimen;
//! p:=list_ptr(just_box);
//! while p<>null do
//!   begin @<Let |d| be the natural width of node |p|;
//!     if the node is ``visible,'' |goto found|;
//!     if the node is glue that stretches or shrinks, set |v:=max_dimen|@>;
//!   if v<max_dimen then v:=v+d;
//!   goto not_found;
//!   found: if v<max_dimen then
//!     begin v:=v+d; w:=v;
//!     end
//!   else  begin w:=max_dimen; goto done;
//!     end;
//!   not_found: p:=link(p);
//!   end;
//! done:
//!
//! @ @<Let |d| be the natural width of node |p|...@>=
//! reswitch: if is_char_node(p) then
//!   begin f:=font(p); d:=char_width(f)(char_info(f)(character(p)));
//!   goto found;
//!   end;
//! case type(p) of
//! hlist_node,vlist_node,rule_node: begin d:=width(p); goto found;
//!   end;
//! ligature_node:@<Make node |p| look like a |char_node|...@>;
//! kern_node,math_node: d:=width(p);
//! glue_node:@<Let |d| be the natural width of this glue; if stretching
//!   or shrinking, set |v:=max_dimen|; |goto found| in the case of leaders@>;
//! whatsit_node: @<Let |d| be the width of the whatsit |p|@>;
//! othercases d:=0
//! endcases
//!
//! @ We need to be careful that |w|, |v|, and |d| do not depend on any |glue_set|
//! values, since such values are subject to system-dependent rounding.
//! System-dependent numbers are not allowed to infiltrate parameters like
//! |pre_display_size|, since \TeX82 is supposed to make the same decisions on all
//! machines.
//!
//! @<Let |d| be the natural width of this glue...@>=
//! begin q:=glue_ptr(p); d:=width(q);
//! if glue_sign(just_box)=stretching then
//!   begin if (glue_order(just_box)=stretch_order(q))and@|
//!      (stretch(q)<>0) then
//!     v:=max_dimen;
//!   end
//! else if glue_sign(just_box)=shrinking then
//!   begin if (glue_order(just_box)=shrink_order(q))and@|
//!      (shrink(q)<>0) then
//!     v:=max_dimen;
//!   end;
//! if subtype(p)>=a_leaders then goto found;
//! end
//!
//! @ A displayed equation is considered to be three lines long, so we
//! calculate the length and offset of line number |prev_graf+2|.
//!
//! @<Calculate the length, |l|, ...@>=
//! if par_shape_ptr=null then
//!   if (hang_indent<>0)and@|
//!    (((hang_after>=0)and(prev_graf+2>hang_after))or@|
//!     (prev_graf+1<-hang_after)) then
//!     begin l:=hsize-abs(hang_indent);
//!     if hang_indent>0 then s:=hang_indent@+else s:=0;
//!     end
//!   else  begin l:=hsize; s:=0;
//!     end
//! else  begin n:=info(par_shape_ptr);
//!   if prev_graf+2>=n then p:=par_shape_ptr+2*n
//!   else p:=par_shape_ptr+2*(prev_graf+2);
//!   s:=mem[p-1].sc; l:=mem[p].sc;
//!   end
//!
//! @ Subformulas of math formulas cause a new level of math mode to be entered,
//! on the semantic nest as well as the save stack. These subformulas arise in
//! several ways: (1)~A left brace by itself indicates the beginning of a
//! subformula that will be put into a box, thereby freezing its glue and
//! preventing line breaks. (2)~A subscript or superscript is treated as a
//! subformula if it is not a single character; the same applies to
//! the nucleus of things like \.{\\underline}. (3)~The \.{\\left} primitive
//! initiates a subformula that will be terminated by a matching \.{\\right}.
//! The group codes placed on |save_stack| in these three cases are
//! |math_group|, |math_group|, and |math_left_group|, respectively.
//!
//! Here is the code that handles case (1); the other cases are not quite as
//! trivial, so we shall consider them later.
//!
//! @<Cases of |main_control| that build...@>=
//! mmode+left_brace: begin tail_append(new_noad);
//!   back_input; scan_math(nucleus(tail));
//!   end;
//!
//! @ Recall that the |nucleus|, |subscr|, and |supscr| fields in a noad are
//! broken down into subfields called |math_type| and either |info| or
//! |(fam,character)|. The job of |scan_math| is to figure out what to place
//! in one of these principal fields; it looks at the subformula that
//! comes next in the input, and places an encoding of that subformula
//! into a given word of |mem|.
//!
//! @d fam_in_range==((cur_fam>=0)and(cur_fam<16))
//!
//! @<Declare act...@>=
//! procedure scan_math(@!p:pointer);
//! label restart,reswitch,exit;
//! var c:integer; {math character code}
//! begin restart:@<Get the next non-blank non-relax...@>;
//! reswitch:case cur_cmd of
//! letter,other_char,char_given: begin c:=ho(math_code(cur_chr));
//!     if c=@'100000 then
//!       begin @<Treat |cur_chr| as an active character@>;
//!       goto restart;
//!       end;
//!     end;
//! char_num: begin scan_char_num; cur_chr:=cur_val; cur_cmd:=char_given;
//!   goto reswitch;
//!   end;
//! math_char_num: begin scan_fifteen_bit_int; c:=cur_val;
//!   end;
//! math_given: c:=cur_chr;
//! delim_num: begin scan_twenty_seven_bit_int; c:=cur_val div @'10000;
//!   end;
//! othercases @<Scan a subformula enclosed in braces and |return|@>
//! endcases;@/
//! math_type(p):=math_char; character(p):=qi(c mod 256);
//! if (c>=var_code)and fam_in_range then fam(p):=cur_fam
//! else fam(p):=(c div 256) mod 16;
//! exit:end;
//!
//! @ An active character that is an |outer_call| is allowed here.
//!
//! @<Treat |cur_chr|...@>=
//! begin cur_cs:=cur_chr+active_base;
//! cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
//! x_token; back_input;
//! end
//!
//! @ The pointer |p| is placed on |save_stack| while a complex subformula
//! is being scanned.
//!
//! @<Scan a subformula...@>=
//! begin back_input; scan_left_brace;@/
//! saved(0):=p; incr(save_ptr); push_math(math_group); return;
//! end
//!
//! @ The simplest math formula is, of course, `\.{\${ }\$}', when no noads are
//! generated. The next simplest cases involve a single character, e.g.,
//! `\.{\$x\$}'. Even though such cases may not seem to be very interesting,
//! the reader can perhaps understand how happy the author was when `\.{\$x\$}'
//! was first properly typeset by \TeX. The code in this section was used.
//! @^Knuth, Donald Ervin@>
//!
//! @<Cases of |main_control| that build...@>=
//! mmode+letter,mmode+other_char,mmode+char_given:
//!   set_math_char(ho(math_code(cur_chr)));
//! mmode+char_num: begin scan_char_num; cur_chr:=cur_val;
//!   set_math_char(ho(math_code(cur_chr)));
//!   end;
//! mmode+math_char_num: begin scan_fifteen_bit_int; set_math_char(cur_val);
//!   end;
//! mmode+math_given: set_math_char(cur_chr);
//! mmode+delim_num: begin scan_twenty_seven_bit_int;
//!   set_math_char(cur_val div @'10000);
//!   end;
//!
//! @ The |set_math_char| procedure creates a new noad appropriate to a given
//! math code, and appends it to the current mlist. However, if the math code
//! is sufficiently large, the |cur_chr| is treated as an active character and
//! nothing is appended.
//!
//! @<Declare act...@>=
//! procedure set_math_char(@!c:integer);
//! var p:pointer; {the new noad}
//! begin if c>=@'100000 then
//!   @<Treat |cur_chr|...@>
//! else  begin p:=new_noad; math_type(nucleus(p)):=math_char;
//!   character(nucleus(p)):=qi(c mod 256);
//!   fam(nucleus(p)):=(c div 256) mod 16;
//!   if c>=var_code then
//!     begin if fam_in_range then fam(nucleus(p)):=cur_fam;
//!     type(p):=ord_noad;
//!     end
//!   else  type(p):=ord_noad+(c div @'10000);
//!   link(tail):=p; tail:=p;
//!   end;
//! end;
//!
//! @ Primitive math operators like \.{\\mathop} and \.{\\underline} are given
//! the command code |math_comp|, supplemented by the noad type that they
//! generate.
//!
//! @<Put each...@>=
//! primitive("mathord",math_comp,ord_noad);
//! @!@:math_ord_}{\.{\\mathord} primitive@>
//! primitive("mathop",math_comp,op_noad);
//! @!@:math_op_}{\.{\\mathop} primitive@>
//! primitive("mathbin",math_comp,bin_noad);
//! @!@:math_bin_}{\.{\\mathbin} primitive@>
//! primitive("mathrel",math_comp,rel_noad);
//! @!@:math_rel_}{\.{\\mathrel} primitive@>
//! primitive("mathopen",math_comp,open_noad);
//! @!@:math_open_}{\.{\\mathopen} primitive@>
//! primitive("mathclose",math_comp,close_noad);
//! @!@:math_close_}{\.{\\mathclose} primitive@>
//! primitive("mathpunct",math_comp,punct_noad);
//! @!@:math_punct_}{\.{\\mathpunct} primitive@>
//! primitive("mathinner",math_comp,inner_noad);
//! @!@:math_inner_}{\.{\\mathinner} primitive@>
//! primitive("underline",math_comp,under_noad);
//! @!@:underline_}{\.{\\underline} primitive@>
//! primitive("overline",math_comp,over_noad);@/
//! @!@:overline_}{\.{\\overline} primitive@>
//! primitive("displaylimits",limit_switch,normal);
//! @!@:display_limits_}{\.{\\displaylimits} primitive@>
//! primitive("limits",limit_switch,limits);
//! @!@:limits_}{\.{\\limits} primitive@>
//! primitive("nolimits",limit_switch,no_limits);
//! @!@:no_limits_}{\.{\\nolimits} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! math_comp: case chr_code of
//!   ord_noad: print_esc("mathord");
//!   op_noad: print_esc("mathop");
//!   bin_noad: print_esc("mathbin");
//!   rel_noad: print_esc("mathrel");
//!   open_noad: print_esc("mathopen");
//!   close_noad: print_esc("mathclose");
//!   punct_noad: print_esc("mathpunct");
//!   inner_noad: print_esc("mathinner");
//!   under_noad: print_esc("underline");
//!   othercases print_esc("overline")
//!   endcases;
//! limit_switch: if chr_code=limits then print_esc("limits")
//!   else if chr_code=no_limits then print_esc("nolimits")
//!   else print_esc("displaylimits");
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+math_comp: begin tail_append(new_noad);
//!   type(tail):=cur_chr; scan_math(nucleus(tail));
//!   end;
//! mmode+limit_switch: math_limit_switch;
//!
//! @ @<Declare act...@>=
//! procedure math_limit_switch;
//! label exit;
//! begin if head<>tail then if type(tail)=op_noad then
//!   begin subtype(tail):=cur_chr; return;
//!   end;
//! print_err("Limit controls must follow a math operator");
//! @.Limit controls must follow...@>
//! help1("I'm ignoring this misplaced \limits or \nolimits command."); error;
//! exit:end;
//!
//! @ Delimiter fields of noads are filled in by the |scan_delimiter| routine.
//! The first parameter of this procedure is the |mem| address where the
//! delimiter is to be placed; the second tells if this delimiter follows
//! \.{\\radical} or not.
//!
//! @<Declare act...@>=
//! procedure scan_delimiter(@!p:pointer;@!r:boolean);
//! begin if r then scan_twenty_seven_bit_int
//! else  begin @<Get the next non-blank non-relax...@>;
//!   case cur_cmd of
//!   letter,other_char: cur_val:=del_code(cur_chr);
//!   delim_num: scan_twenty_seven_bit_int;
//!   othercases cur_val:=-1
//!   endcases;
//!   end;
//! if cur_val<0 then @<Report that an invalid delimiter code is being changed
//!    to null; set~|cur_val:=0|@>;
//! small_fam(p):=(cur_val div @'4000000) mod 16;
//! small_char(p):=qi((cur_val div @'10000) mod 256);
//! large_fam(p):=(cur_val div 256) mod 16;
//! large_char(p):=qi(cur_val mod 256);
//! end;
//!
//! @ @<Report that an invalid delimiter...@>=
//! begin print_err("Missing delimiter (. inserted)");
//! @.Missing delimiter...@>
//! help6("I was expecting to see something like `(' or `\{' or")@/
//!   ("`\}' here. If you typed, e.g., `{' instead of `\{', you")@/
//!   ("should probably delete the `{' by typing `1' now, so that")@/
//!   ("braces don't get unbalanced. Otherwise just proceed.")@/
//!   ("Acceptable delimiters are characters whose \delcode is")@/
//!   ("nonnegative, or you can use `\delimiter <delimiter code>'.");
//! back_error; cur_val:=0;
//! end
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+radical:math_radical;
//!
//! @ @<Declare act...@>=
//! procedure math_radical;
//! begin tail_append(get_node(radical_noad_size));
//! type(tail):=radical_noad; subtype(tail):=normal;
//! mem[nucleus(tail)].hh:=empty_field;
//! mem[subscr(tail)].hh:=empty_field;
//! mem[supscr(tail)].hh:=empty_field;
//! scan_delimiter(left_delimiter(tail),true); scan_math(nucleus(tail));
//! end;
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+accent,mmode+math_accent:math_ac;
//!
//! @ @<Declare act...@>=
//! procedure math_ac;
//! begin if cur_cmd=accent then
//!   @<Complain that the user should have said \.{\\mathaccent}@>;
//! tail_append(get_node(accent_noad_size));
//! type(tail):=accent_noad; subtype(tail):=normal;
//! mem[nucleus(tail)].hh:=empty_field;
//! mem[subscr(tail)].hh:=empty_field;
//! mem[supscr(tail)].hh:=empty_field;
//! math_type(accent_chr(tail)):=math_char;
//! scan_fifteen_bit_int;
//! character(accent_chr(tail)):=qi(cur_val mod 256);
//! if (cur_val>=var_code)and fam_in_range then fam(accent_chr(tail)):=cur_fam
//! else fam(accent_chr(tail)):=(cur_val div 256) mod 16;
//! scan_math(nucleus(tail));
//! end;
//!
//! @ @<Complain that the user should have said \.{\\mathaccent}@>=
//! begin print_err("Please use "); print_esc("mathaccent");
//! print(" for accents in math mode");
//! @.Please use \\mathaccent...@>
//! help2("I'm changing \accent to \mathaccent here; wish me luck.")@/
//!   ("(Accents are not the same in formulas as they are in text.)");
//! error;
//! end
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+vcenter: begin scan_spec(vcenter_group,false); normal_paragraph;
//!   push_nest; mode:=-vmode; prev_depth:=ignore_depth;
//!   if every_vbox<>null then begin_token_list(every_vbox,every_vbox_text);
//!   end;
//!
//! @ @<Cases of |handle...@>=
//! vcenter_group: begin end_graf; unsave; save_ptr:=save_ptr-2;
//!   p:=vpack(link(head),saved(1),saved(0)); pop_nest;
//!   tail_append(new_noad); type(tail):=vcenter_noad;
//!   math_type(nucleus(tail)):=sub_box; info(nucleus(tail)):=p;
//!   end;
//!
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
//! @ Here is the only way out of math mode.
//!
//! @<Cases of |main_control| that build...@>=
//! mmode+math_shift: if cur_group=math_shift_group then after_math
//!   else off_save;
//!
//! @ @<Declare act...@>=
//! procedure after_math;
//! var l:boolean; {`\.{\\leqno}' instead of `\.{\\eqno}'}
//! @!danger:boolean; {not enough symbol fonts are present}
//! @!m:integer; {|mmode| or |-mmode|}
//! @!p:pointer; {the formula}
//! @!a:pointer; {box containing equation number}
//! @<Local variables for finishing a displayed formula@>@;
//! begin danger:=false;
//! @<Check that the necessary fonts for math symbols are present;
//!   if not, flush the current math lists and set |danger:=true|@>;
//! m:=mode; l:=false; p:=fin_mlist(null); {this pops the nest}
//! if mode=-m then {end of equation number}
//!   begin @<Check that another \.\$ follows@>;
//!   cur_mlist:=p; cur_style:=text_style; mlist_penalties:=false;
//!   mlist_to_hlist; a:=hpack(link(temp_head),natural);
//!   unsave; decr(save_ptr); {now |cur_group=math_shift_group|}
//!   if saved(0)=1 then l:=true;
//!   danger:=false;
//!   @<Check that the necessary fonts for math symbols are present;
//!     if not, flush the current math lists and set |danger:=true|@>;
//!   m:=mode; p:=fin_mlist(null);
//!   end
//! else a:=null;
//! if m<0 then @<Finish math in text@>
//! else  begin if a=null then @<Check that another \.\$ follows@>;
//!   @<Finish displayed math@>;
//!   end;
//! end;
//!
//! @ @<Check that the necessary fonts...@>=
//! if (font_params[fam_fnt(2+text_size)]<total_mathsy_params)or@|
//!    (font_params[fam_fnt(2+script_size)]<total_mathsy_params)or@|
//!    (font_params[fam_fnt(2+script_script_size)]<total_mathsy_params) then
//!   begin print_err("Math formula deleted: Insufficient symbol fonts");@/
//! @.Math formula deleted...@>
//!   help3("Sorry, but I can't typeset math unless \textfont 2")@/
//!     ("and \scriptfont 2 and \scriptscriptfont 2 have all")@/
//!     ("the \fontdimen values needed in math symbol fonts.");
//!   error; flush_math; danger:=true;
//!   end
//! else if (font_params[fam_fnt(3+text_size)]<total_mathex_params)or@|
//!    (font_params[fam_fnt(3+script_size)]<total_mathex_params)or@|
//!    (font_params[fam_fnt(3+script_script_size)]<total_mathex_params) then
//!   begin print_err("Math formula deleted: Insufficient extension fonts");@/
//!   help3("Sorry, but I can't typeset math unless \textfont 3")@/
//!     ("and \scriptfont 3 and \scriptscriptfont 3 have all")@/
//!     ("the \fontdimen values needed in math extension fonts.");
//!   error; flush_math; danger:=true;
//!   end
//!
//! @ The |unsave| is done after everything else here; hence an appearance of
//! `\.{\\mathsurround}' inside of `\.{\$...\$}' affects the spacing at these
//! particular \.\$'s. This is consistent with the conventions of
//! `\.{\$\$...\$\$}', since `\.{\\abovedisplayskip}' inside a display affects the
//! space above that display.
//!
//! @<Finish math in text@>=
//! begin tail_append(new_math(math_surround,before));
//! cur_mlist:=p; cur_style:=text_style; mlist_penalties:=(mode>0); mlist_to_hlist;
//! link(tail):=link(temp_head);
//! while link(tail)<>null do tail:=link(tail);
//! tail_append(new_math(math_surround,after));
//! space_factor:=1000; unsave;
//! end
//!
//! @ \TeX\ gets to the following part of the program when the first `\.\$' ending
//! a display has been scanned.
//!
//! @<Check that another \.\$ follows@>=
//! begin get_x_token;
//! if cur_cmd<>math_shift then
//!   begin print_err("Display math should end with $$");
//! @.Display math...with \$\$@>
//!   help2("The `$' that I just saw supposedly matches a previous `$$'.")@/
//!     ("So I shall assume that you typed `$$' both times.");
//!   back_error;
//!   end;
//! end
//!
//! @ We have saved the worst for last: The fussiest part of math mode processing
//! occurs when a displayed formula is being centered and placed with an optional
//! equation number.
//!
//! @<Local variables for finishing...@>=
//! @!b:pointer; {box containing the equation}
//! @!w:scaled; {width of the equation}
//! @!z:scaled; {width of the line}
//! @!e:scaled; {width of equation number}
//! @!q:scaled; {width of equation number plus space to separate from equation}
//! @!d:scaled; {displacement of equation in the line}
//! @!s:scaled; {move the line right this much}
//! @!g1,@!g2:small_number; {glue parameter codes for before and after}
//! @!r:pointer; {kern node used to position the display}
//! @!t:pointer; {tail of adjustment list}
