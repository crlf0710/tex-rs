//! @ @<Declare act...@>=
//! procedure head_for_vmode;
//! begin if mode<0 then
//!   if cur_cmd<>hrule then off_save
//!   else  begin print_err("You can't use `");
//!     print_esc("hrule"); print("' here except with leaders");
//! @.You can't use \\hrule...@>
//!     help2("To put a horizontal rule in an hbox or an alignment,")@/
//!       ("you should use \leaders or \hrulefill (see The TeXbook).");
//!     error;
//!     end
//! else  begin back_input; cur_tok:=par_token; back_input; token_type:=inserted;
//!   end;
//! end;
//!
//! @ @<Declare act...@>=
//! procedure end_graf;
//! begin if mode=hmode then
//!   begin if head=tail then pop_nest {null paragraphs are ignored}
//!   else line_break(widow_penalty);
//!   normal_paragraph;
//!   error_count:=0;
//!   end;
//! end;
//!
//! @ Insertion and adjustment and mark nodes are constructed by the following
//! pieces of the program.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(insert),hmode+vadjust,mmode+vadjust: begin_insert_or_adjust;
//! any_mode(mark): make_mark;
//!
//! @ @<Forbidden...@>=
//! vmode+vadjust,
//!
//! @ @<Declare act...@>=
//! procedure begin_insert_or_adjust;
//! begin if cur_cmd=vadjust then cur_val:=255
//! else  begin scan_eight_bit_int;
//!   if cur_val=255 then
//!     begin print_err("You can't "); print_esc("insert"); print_int(255);
//! @.You can't \\insert255@>
//!     help1("I'm changing to \insert0; box 255 is special.");
//!     error; cur_val:=0;
//!     end;
//!   end;
//! saved(0):=cur_val; incr(save_ptr);
//! new_save_level(insert_group); scan_left_brace; normal_paragraph;
//! push_nest; mode:=-vmode; prev_depth:=ignore_depth;
//! end;
//!
//! @ @<Cases of |handle...@>=
//! insert_group: begin end_graf; q:=split_top_skip; add_glue_ref(q);
//!   d:=split_max_depth; f:=floating_penalty; unsave; decr(save_ptr);
//!   {now |saved(0)| is the insertion number, or 255 for |vadjust|}
//!   p:=vpack(link(head),natural); pop_nest;
//!   if saved(0)<255 then
//!     begin tail_append(get_node(ins_node_size));
//!     type(tail):=ins_node; subtype(tail):=qi(saved(0));
//!     height(tail):=height(p)+depth(p); ins_ptr(tail):=list_ptr(p);
//!     split_top_ptr(tail):=q; depth(tail):=d; float_cost(tail):=f;
//!     end
//!   else  begin tail_append(get_node(small_node_size));
//!     type(tail):=adjust_node;@/
//!     subtype(tail):=0; {the |subtype| is not used}
//!     adjust_ptr(tail):=list_ptr(p); delete_glue_ref(q);
//!     end;
//!   free_node(p,box_node_size);
//!   if nest_ptr=0 then build_page;
//!   end;
//! output_group: @<Resume the page builder...@>;
//!
//! @ @<Declare act...@>=
//! procedure make_mark;
//! var p:pointer; {new node}
//! begin p:=scan_toks(false,true); p:=get_node(small_node_size);
//! type(p):=mark_node; subtype(p):=0; {the |subtype| is not used}
//! mark_ptr(p):=def_ref; link(tail):=p; tail:=p;
//! end;
//!
//! @ Penalty nodes get into a list via the |break_penalty| command.
//! @^penalties@>
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(break_penalty): append_penalty;
//!
//! @ @<Declare action...@>=
//! procedure append_penalty;
//! begin scan_int; tail_append(new_penalty(cur_val));
//! if mode=vmode then build_page;
//! end;
//!
//! @ The |remove_item| command removes a penalty, kern, or glue node if it
//! appears at the tail of the current list, using a brute-force linear scan.
//! Like \.{\\lastbox}, this command is not allowed in vertical mode (except
//! internal vertical mode), since the current list in vertical mode is sent
//! to the page builder.  But if we happen to be able to implement it in
//! vertical mode, we do.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(remove_item): delete_last;
//!
//! @ When |delete_last| is called, |cur_chr| is the |type| of node that
//! will be deleted, if present.
//!
//! @<Declare action...@>=
//! procedure delete_last;
//! label exit;
//! var @!p,@!q:pointer; {run through the current list}
//! @!m:quarterword; {the length of a replacement list}
//! begin if (mode=vmode)and(tail=head) then
//!   @<Apologize for inability to do the operation now,
//!     unless \.{\\unskip} follows non-glue@>
//! else  begin if not is_char_node(tail) then if type(tail)=cur_chr then
//!     begin q:=head;
//!     repeat p:=q;
//!     if not is_char_node(q) then if type(q)=disc_node then
//!       begin for m:=1 to replace_count(q) do p:=link(p);
//!       if p=tail then return;
//!       end;
//!     q:=link(p);
//!     until q=tail;
//!     link(p):=null; flush_node_list(tail); tail:=p;
//!     end;
//!   end;
//! exit:end;
//!
//! @ @<Apologize for inability to do the operation...@>=
//! begin if (cur_chr<>glue_node)or(last_glue<>max_halfword) then
//!   begin you_cant;
//!   help2("Sorry...I usually can't take things from the current page.")@/
//!     ("Try `I\vskip-\lastskip' instead.");
//!   if cur_chr=kern_node then help_line[0]:=
//!     ("Try `I\kern-\lastkern' instead.")
//!   else if cur_chr<>glue_node then help_line[0]:=@|
//!     ("Perhaps you can make the output routine do it.");
//!   error;
//!   end;
//! end
//!
//! @ @<Put each...@>=
//! primitive("unpenalty",remove_item,penalty_node);@/
//! @!@:un_penalty_}{\.{\\unpenalty} primitive@>
//! primitive("unkern",remove_item,kern_node);@/
//! @!@:un_kern_}{\.{\\unkern} primitive@>
//! primitive("unskip",remove_item,glue_node);@/
//! @!@:un_skip_}{\.{\\unskip} primitive@>
//! primitive("unhbox",un_hbox,box_code);@/
//! @!@:un_hbox_}{\.{\\unhbox} primitive@>
//! primitive("unhcopy",un_hbox,copy_code);@/
//! @!@:un_hcopy_}{\.{\\unhcopy} primitive@>
//! primitive("unvbox",un_vbox,box_code);@/
//! @!@:un_vbox_}{\.{\\unvbox} primitive@>
//! primitive("unvcopy",un_vbox,copy_code);@/
//! @!@:un_vcopy_}{\.{\\unvcopy} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! remove_item: if chr_code=glue_node then print_esc("unskip")
//!   else if chr_code=kern_node then print_esc("unkern")
//!   else print_esc("unpenalty");
//! un_hbox: if chr_code=copy_code then print_esc("unhcopy")
//!   else print_esc("unhbox");
//! un_vbox: if chr_code=copy_code then print_esc("unvcopy")
//!   else print_esc("unvbox");
//!
//! @ The |un_hbox| and |un_vbox| commands unwrap one of the 256 current boxes.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+un_vbox,hmode+un_hbox,mmode+un_hbox: unpackage;
//!
//! @ @<Declare act...@>=
//! procedure unpackage;
//! label exit;
//! var p:pointer; {the box}
//! @!c:box_code..copy_code; {should we copy?}
//! begin c:=cur_chr; scan_eight_bit_int; p:=box(cur_val);
//! if p=null then return;
//! if (abs(mode)=mmode)or((abs(mode)=vmode)and(type(p)<>vlist_node))or@|
//!    ((abs(mode)=hmode)and(type(p)<>hlist_node)) then
//!   begin print_err("Incompatible list can't be unboxed");
//! @.Incompatible list...@>
//!   help3("Sorry, Pandora. (You sneaky devil.)")@/
//!   ("I refuse to unbox an \hbox in vertical mode or vice versa.")@/
//!   ("And I can't open any boxes in math mode.");@/
//!   error; return;
//!   end;
//! if c=copy_code then link(tail):=copy_node_list(list_ptr(p))
//! else  begin link(tail):=list_ptr(p); box(cur_val):=null;
//!   free_node(p,box_node_size);
//!   end;
//! while link(tail)<>null do tail:=link(tail);
//! exit:end;
//!
//! @ @<Forbidden...@>=vmode+ital_corr,
//!
//! @ Italic corrections are converted to kern nodes when the |ital_corr| command
//! follows a character. In math mode the same effect is achieved by appending
//! a kern of zero here, since italic corrections are supplied later.
//!
//! @<Cases of |main_control| that build...@>=
//! hmode+ital_corr: append_italic_correction;
//! mmode+ital_corr: tail_append(new_kern(0));
//!
//! @ @<Declare act...@>=
//! procedure append_italic_correction;
//! label exit;
//! var p:pointer; {|char_node| at the tail of the current list}
//! @!f:internal_font_number; {the font in the |char_node|}
//! begin if tail<>head then
//!   begin if is_char_node(tail) then p:=tail
//!   else if type(tail)=ligature_node then p:=lig_char(tail)
//!   else return;
//!   f:=font(p);
//!   tail_append(new_kern(char_italic(f)(char_info(f)(character(p)))));
//!   subtype(tail):=explicit;
//!   end;
//! exit:end;
//!
//! @ Discretionary nodes are easy in the common case `\.{\\-}', but in the
//! general case we must process three braces full of items.
//!
//! @<Put each...@>=
//! primitive("-",discretionary,1);
//! @!@:Single-character primitives -}{\quad\.{\\-}@>
//! primitive("discretionary",discretionary,0);
//! @!@:discretionary_}{\.{\\discretionary} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! discretionary: if chr_code=1 then
//!   print_esc("-")@+else print_esc("discretionary");
//!
//! @ @<Cases of |main_control| that build...@>=
//! hmode+discretionary,mmode+discretionary: append_discretionary;
//!
//! @ The space factor does not change when we append a discretionary node,
//! but it starts out as 1000 in the subsidiary lists.
//!
//! @<Declare act...@>=
//! procedure append_discretionary;
//! var c:integer; {hyphen character}
//! begin tail_append(new_disc);
//! if cur_chr=1 then
//!   begin c:=hyphen_char[cur_font];
//!   if c>=0 then if c<256 then pre_break(tail):=new_character(cur_font,c);
//!   end
//! else  begin incr(save_ptr); saved(-1):=0; new_save_level(disc_group);
//!   scan_left_brace; push_nest; mode:=-hmode; space_factor:=1000;
//!   end;
//! end;
//!
//! @ The three discretionary lists are constructed somewhat as if they were
//! hboxes. A~subroutine called |build_discretionary| handles the transitions.
//! (This is sort of fun.)
//!
//! @<Cases of |handle...@>=
//! disc_group: build_discretionary;
//!
//! @ @<Declare act...@>=
//! procedure build_discretionary;
//! label done,exit;
//! var p,@!q:pointer; {for link manipulation}
//! @!n:integer; {length of discretionary list}
//! begin unsave;
//! @<Prune the current list, if necessary, until it contains only
//!   |char_node|, |kern_node|, |hlist_node|, |vlist_node|, |rule_node|,
//!   and |ligature_node| items; set |n| to the length of the list,
//!   and set |q| to the list's tail@>;
//! p:=link(head); pop_nest;
//! case saved(-1) of
//! 0:pre_break(tail):=p;
//! 1:post_break(tail):=p;
//! 2:@<Attach list |p| to the current list, and record its length;
//!   then finish up and |return|@>;
//! end; {there are no other cases}
//! incr(saved(-1)); new_save_level(disc_group); scan_left_brace;
//! push_nest; mode:=-hmode; space_factor:=1000;
//! exit:end;
//!
//! @ @<Attach list |p| to the current...@>=
//! begin if (n>0)and(abs(mode)=mmode) then
//!   begin print_err("Illegal math "); print_esc("discretionary");
//! @.Illegal math \\disc...@>
//!   help2("Sorry: The third part of a discretionary break must be")@/
//!   ("empty, in math formulas. I had to delete your third part.");
//!   flush_node_list(p); n:=0; error;
//!   end
//! else link(tail):=p;
//! if n<=max_quarterword then replace_count(tail):=n
//! else  begin print_err("Discretionary list is too long");
//! @.Discretionary list is too long@>
//!   help2("Wow---I never thought anybody would tweak me here.")@/
//!   ("You can't seriously need such a huge discretionary list?");
//!   error;
//!   end;
//! if n>0 then tail:=q;
//! decr(save_ptr); return;
//! end
//!
//! @ During this loop, |p=link(q)| and there are |n| items preceding |p|.
//!
//! @<Prune the current list, if necessary...@>=
//! q:=head; p:=link(q); n:=0;
//! while p<>null do
//!   begin if not is_char_node(p) then if type(p)>rule_node then
//!     if type(p)<>kern_node then if type(p)<>ligature_node then
//!       begin print_err("Improper discretionary list");
//! @.Improper discretionary list@>
//!       help1("Discretionary lists must contain only boxes and kerns.");@/
//!       error;
//!       begin_diagnostic;
//!       print_nl("The following discretionary sublist has been deleted:");
//! @.The following...deleted@>
//!       show_box(p);
//!       end_diagnostic(true);
//!       flush_node_list(p); link(q):=null; goto done;
//!       end;
//!   q:=p; p:=link(q); incr(n);
//!   end;
//! done:
//!
//! @ We need only one more thing to complete the horizontal mode routines, namely
//! the \.{\\accent} primitive.
//!
//! @<Cases of |main_control| that build...@>=
//! hmode+accent: make_accent;
//!
//! @ The positioning of accents is straightforward but tedious. Given an accent
//! of width |a|, designed for characters of height |x| and slant |s|;
//! and given a character of width |w|, height |h|, and slant |t|: We will shift
//! the accent down by |x-h|, and we will insert kern nodes that have the effect of
//! centering the accent over the character and shifting the accent to the
//! right by $\delta={1\over2}(w-a)+h\cdot t-x\cdot s$.  If either character is
//! absent from the font, we will simply use the other, without shifting.
//!
//! @<Declare act...@>=
//! procedure make_accent;
//! var s,@!t: real; {amount of slant}
//! @!p,@!q,@!r:pointer; {character, box, and kern nodes}
//! @!f:internal_font_number; {relevant font}
//! @!a,@!h,@!x,@!w,@!delta:scaled; {heights and widths, as explained above}
//! @!i:four_quarters; {character information}
//! begin scan_char_num; f:=cur_font; p:=new_character(f,cur_val);
//! if p<>null then
//!   begin x:=x_height(f); s:=slant(f)/float_constant(65536);
//! @^real division@>
//!   a:=char_width(f)(char_info(f)(character(p)));@/
//!   do_assignments;@/
//!   @<Create a character node |q| for the next character,
//!     but set |q:=null| if problems arise@>;
//!   if q<>null then @<Append the accent with appropriate kerns,
//!       then set |p:=q|@>;
//!   link(tail):=p; tail:=p; space_factor:=1000;
//!   end;
//! end;
//!
//! @ @<Create a character node |q| for the next...@>=
//! q:=null; f:=cur_font;
//! if (cur_cmd=letter)or(cur_cmd=other_char)or(cur_cmd=char_given) then
//!   q:=new_character(f,cur_chr)
//! else if cur_cmd=char_num then
//!   begin scan_char_num; q:=new_character(f,cur_val);
//!   end
//! else back_input
//!
//! @ The kern nodes appended here must be distinguished from other kerns, lest
//! they be wiped away by the hyphenation algorithm or by a previous line break.
//!
//! The two kerns are computed with (machine-dependent) |real| arithmetic, but
//! their sum is machine-independent; the net effect is machine-independent,
//! because the user cannot remove these nodes nor access them via \.{\\lastkern}.
//!
//! @<Append the accent with appropriate kerns...@>=
//! begin t:=slant(f)/float_constant(65536);
//! @^real division@>
//! i:=char_info(f)(character(q));
//! w:=char_width(f)(i); h:=char_height(f)(height_depth(i));
//! if h<>x then {the accent must be shifted up or down}
//!   begin p:=hpack(p,natural); shift_amount(p):=x-h;
//!   end;
//! delta:=round((w-a)/float_constant(2)+h*t-x*s);
//! @^real multiplication@>
//! @^real addition@>
//! r:=new_kern(delta); subtype(r):=acc_kern; link(tail):=r; link(r):=p;
//! tail:=new_kern(-a-delta); subtype(tail):=acc_kern; link(p):=tail; p:=q;
//! end
//!
//! @ When `\.{\\cr}' or `\.{\\span}' or a tab mark comes through the scanner
//! into |main_control|, it might be that the user has foolishly inserted
//! one of them into something that has nothing to do with alignment. But it is
//! far more likely that a left brace or right brace has been omitted, since
//! |get_next| takes actions appropriate to alignment only when `\.{\\cr}'
//! or `\.{\\span}' or tab marks occur with |align_state=0|. The following
//! program attempts to make an appropriate recovery.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(car_ret), any_mode(tab_mark): align_error;
//! any_mode(no_align): no_align_error;
//! any_mode(omit): omit_error;
//!
//! @ @<Declare act...@>=
//! procedure align_error;
//! begin if abs(align_state)>2 then
//!   @<Express consternation over the fact that no alignment is in progress@>
//! else  begin back_input;
//!   if align_state<0 then
//!     begin print_err("Missing { inserted");
//! @.Missing \{ inserted@>
//!     incr(align_state); cur_tok:=left_brace_token+"{";
//!     end
//!   else  begin print_err("Missing } inserted");
//! @.Missing \} inserted@>
//!     decr(align_state); cur_tok:=right_brace_token+"}";
//!     end;
//!   help3("I've put in what seems to be necessary to fix")@/
//!     ("the current column of the current alignment.")@/
//!     ("Try to go on, since this might almost work."); ins_error;
//!   end;
//! end;
//!
//! @ @<Express consternation...@>=
//! begin print_err("Misplaced "); print_cmd_chr(cur_cmd,cur_chr);
//! @.Misplaced \&@>
//! @.Misplaced \\span@>
//! @.Misplaced \\cr@>
//! if cur_tok=tab_token+"&" then
//!   begin help6("I can't figure out why you would want to use a tab mark")@/
//!   ("here. If you just want an ampersand, the remedy is")@/
//!   ("simple: Just type `I\&' now. But if some right brace")@/
//!   ("up above has ended a previous alignment prematurely,")@/
//!   ("you're probably due for more error messages, and you")@/
//!   ("might try typing `S' now just to see what is salvageable.");
//!   end
//! else  begin help5("I can't figure out why you would want to use a tab mark")@/
//!   ("or \cr or \span just now. If something like a right brace")@/
//!   ("up above has ended a previous alignment prematurely,")@/
//!   ("you're probably due for more error messages, and you")@/
//!   ("might try typing `S' now just to see what is salvageable.");
//!   end;
//! error;
//! end
//!
//! @ The help messages here contain a little white lie, since \.{\\noalign}
//! and \.{\\omit} are allowed also after `\.{\\noalign\{...\}}'.
//!
//! @<Declare act...@>=
//! procedure no_align_error;
//! begin print_err("Misplaced "); print_esc("noalign");
//! @.Misplaced \\noalign@>
//! help2("I expect to see \noalign only after the \cr of")@/
//!   ("an alignment. Proceed, and I'll ignore this case."); error;
//! end;
//! procedure omit_error;
//! begin print_err("Misplaced "); print_esc("omit");
//! @.Misplaced \\omit@>
//! help2("I expect to see \omit only after tab marks or the \cr of")@/
//!   ("an alignment. Proceed, and I'll ignore this case."); error;
//! end;
//!
//! @ We've now covered most of the abuses of \.{\\halign} and \.{\\valign}.
//! Let's take a look at what happens when they are used correctly.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+halign,hmode+valign:init_align;
//! mmode+halign: if privileged then
//!   if cur_group=math_shift_group then init_align
//!   else off_save;
//! vmode+endv,hmode+endv: do_endv;
//!
//! @ An |align_group| code is supposed to remain on the |save_stack|
//! during an entire alignment, until |fin_align| removes it.
//!
//! A devious user might force an |endv| command to occur just about anywhere;
//! we must defeat such hacks.
//!
//! @<Declare act...@>=
//! procedure do_endv;
//! begin base_ptr:=input_ptr; input_stack[base_ptr]:=cur_input;
//! while (input_stack[base_ptr].index_field<>v_template) and
//!       (input_stack[base_ptr].loc_field=null) and
//!       (input_stack[base_ptr].state_field=token_list) do decr(base_ptr);
//! if (input_stack[base_ptr].index_field<>v_template) or
//!       (input_stack[base_ptr].loc_field<>null) or
//!       (input_stack[base_ptr].state_field<>token_list) then
//!   fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//!  if cur_group=align_group then
//!   begin end_graf;
//!   if fin_col then fin_row;
//!   end
//! else off_save;
//! end;
//!
//! @ @<Cases of |handle_right_brace|...@>=
//! align_group: begin back_input; cur_tok:=cs_token_flag+frozen_cr;
//!   print_err("Missing "); print_esc("cr"); print(" inserted");
//! @.Missing \\cr inserted@>
//!   help1("I'm guessing that you meant to end an alignment here.");
//!   ins_error;
//!   end;
//!
//! @ @<Cases of |handle_right_brace|...@>=
//! no_align_group: begin end_graf; unsave; align_peek;
//!   end;
//!
//! @ Finally, \.{\\endcsname} is not supposed to get through to |main_control|.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(end_cs_name): cs_error;
//!
//! @ @<Declare act...@>=
//! procedure cs_error;
//! begin print_err("Extra "); print_esc("endcsname");
//! @.Extra \\endcsname@>
//! help1("I'm ignoring this, since I wasn't doing a \csname.");
//! error;
//! end;
//!
