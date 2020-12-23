//! @ Constructions that require a box are started by calling |scan_box| with
//! a specified context code. The |scan_box| routine verifies
//! that a |make_box| command comes next and then it calls |begin_box|.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+hmove,hmode+vmove,mmode+vmove: begin t:=cur_chr;
//!   scan_normal_dimen;
//!   if t=0 then scan_box(cur_val)@+else scan_box(-cur_val);
//!   end;
//! any_mode(leader_ship): scan_box(leader_flag-a_leaders+cur_chr);
//! any_mode(make_box): begin_box(0);
//!
