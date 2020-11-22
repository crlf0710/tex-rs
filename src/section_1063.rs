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
