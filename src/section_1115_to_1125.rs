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
