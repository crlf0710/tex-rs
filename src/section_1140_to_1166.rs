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
