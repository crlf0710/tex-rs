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
