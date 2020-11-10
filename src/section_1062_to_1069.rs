//! @ Many of the actions related to box-making are triggered by the appearance
//! of braces in the input. For example, when the user says `\.{\\hbox}
//! \.{to} \.{100pt\{$\langle\,\hbox{hlist}\,\rangle$\}}' in vertical mode,
//! the information about the box size (100pt, |exactly|) is put onto |save_stack|
//! with a level boundary word just above it, and |cur_group:=adjusted_hbox_group|;
//! \TeX\ enters restricted horizontal mode to process the hlist. The right
//! brace eventually causes |save_stack| to be restored to its former state,
//! at which time the information about the box size (100pt, |exactly|) is
//! available once again; a box is packaged and we leave restricted horizontal
//! mode, appending the new box to the current list of the enclosing mode
//! (in this case to the current list of vertical mode), followed by any
//! vertical adjustments that were removed from the box by |hpack|.
//!
//! The next few sections of the program are therefore concerned with the
//! treatment of left and right curly braces.
//!
//! @ If a left brace occurs in the middle of a page or paragraph, it simply
//! introduces a new level of grouping, and the matching right brace will not have
//! such a drastic effect. Such grouping affects neither the mode nor the
//! current list.
//!
//! @<Cases of |main_control| that build...@>=
//! non_math(left_brace): new_save_level(simple_group);
//! any_mode(begin_group): new_save_level(semi_simple_group);
//! any_mode(end_group): if cur_group=semi_simple_group then unsave
//!   else off_save;
//!
//! @ We have to deal with errors in which braces and such things are not
//! properly nested. Sometimes the user makes an error of commission by
//! inserting an extra symbol, but sometimes the user makes an error of omission.
//! \TeX\ can't always tell one from the other, so it makes a guess and tries
//! to avoid getting into a loop.
//!
//! The |off_save| routine is called when the current group code is wrong. It tries
//! to insert something into the user's input that will help clean off
//! the top level.
//!
//! @<Declare act...@>=
//! procedure off_save;
//! var p:pointer; {inserted token}
//! begin if cur_group=bottom_level then
//!   @<Drop current token and complain that it was unmatched@>
//! else  begin back_input; p:=get_avail; link(temp_head):=p;
//!   print_err("Missing ");
//!   @<Prepare to insert a token that matches |cur_group|,
//!     and print what it is@>;
//!   print(" inserted"); ins_list(link(temp_head));
//!   help5("I've inserted something that you may have forgotten.")@/
//!   ("(See the <inserted text> above.)")@/
//!   ("With luck, this will get me unwedged. But if you")@/
//!   ("really didn't forget anything, try typing `2' now; then")@/
//!   ("my insertion and my current dilemma will both disappear.");
//!   error;
//!   end;
//! end;
//!
//! @ At this point, |link(temp_head)=p|, a pointer to an empty one-word node.
//!
//! @<Prepare to insert a token that matches |cur_group|...@>=
//! case cur_group of
//! semi_simple_group: begin info(p):=cs_token_flag+frozen_end_group;
//!   print_esc("endgroup");
//! @.Missing \\endgroup inserted@>
//!   end;
//! math_shift_group: begin info(p):=math_shift_token+"$"; print_char("$");
//! @.Missing \$ inserted@>
//!   end;
//! math_left_group: begin info(p):=cs_token_flag+frozen_right; link(p):=get_avail;
//!   p:=link(p); info(p):=other_token+"."; print_esc("right.");
//! @.Missing \\right\hbox{.} inserted@>
//! @^null delimiter@>
//!   end;
//! othercases begin info(p):=right_brace_token+"}"; print_char("}");
//! @.Missing \} inserted@>
//!   end
//! endcases
//!
//! @ @<Drop current token and complain that it was unmatched@>=
//! begin print_err("Extra "); print_cmd_chr(cur_cmd,cur_chr);
//! @.Extra x@>
//! help1("Things are pretty mixed up, but I think the worst is over.");@/
//! error;
//! end
//!
//! @ The routine for a |right_brace| character branches into many subcases,
//! since a variety of things may happen, depending on |cur_group|. Some
//! types of groups are not supposed to be ended by a right brace; error
//! messages are given in hopes of pinpointing the problem. Most branches
//! of this routine will be filled in later, when we are ready to understand
//! them; meanwhile, we must prepare ourselves to deal with such errors.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(right_brace): handle_right_brace;
//!
//! @ @<Declare the procedure called |handle_right_brace|@>=
//! procedure handle_right_brace;
//! var p,@!q:pointer; {for short-term use}
//! @!d:scaled; {holds |split_max_depth| in |insert_group|}
//! @!f:integer; {holds |floating_penalty| in |insert_group|}
//! begin case cur_group of
//! simple_group: unsave;
//! bottom_level: begin print_err("Too many }'s");
//! @.Too many \}'s@>
//!   help2("You've closed more groups than you opened.")@/
//!   ("Such booboos are generally harmless, so keep going."); error;
//!   end;
//! semi_simple_group,math_shift_group,math_left_group: extra_right_brace;
//! @t\4@>@<Cases of |handle_right_brace| where a |right_brace| triggers
//!   a delayed action@>@;
//! othercases confusion("rightbrace")
//! @:this can't happen rightbrace}{\quad rightbrace@>
//! endcases;
//! end;
//!
//! @ @<Declare act...@>=
//! procedure extra_right_brace;
//! begin print_err("Extra }, or forgotten ");
//! @.Extra \}, or forgotten x@>
//! case cur_group of
//! semi_simple_group: print_esc("endgroup");
//! math_shift_group: print_char("$");
//! math_left_group: print_esc("right");
//! end;@/
//! help5("I've deleted a group-closing symbol because it seems to be")@/
//! ("spurious, as in `$x}$'. But perhaps the } is legitimate and")@/
//! ("you forgot something else, as in `\hbox{$x}'. In such cases")@/
//! ("the way to recover is to insert both the forgotten and the")@/
//! ("deleted material, e.g., by typing `I$}'."); error;
//! incr(align_state);
//! end;
//!
