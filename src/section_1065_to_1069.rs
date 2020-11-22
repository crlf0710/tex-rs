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
