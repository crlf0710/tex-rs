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
