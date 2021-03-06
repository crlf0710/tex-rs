//! @ The calculations related to leaders require a bit of care. First, in the
//! case of |a_leaders| (aligned leaders), we want to move |cur_h| to
//! |left_edge| plus the smallest multiple of |leader_wd| for which the result
//! is not less than the current value of |cur_h|; i.e., |cur_h| should become
//! $|left_edge|+|leader_wd|\times\lceil
//! (|cur_h|-|left_edge|)/|leader_wd|\rceil$.  The program here should work in
//! all cases even though some implementations of \PASCAL\ give nonstandard
//! results for the |div| operation when |cur_h| is less than |left_edge|.
//!
//! In the case of |c_leaders| (centered leaders), we want to increase |cur_h|
//! by half of the excess space not occupied by the leaders; and in the
//! case of |x_leaders| (expanded leaders) we increase |cur_h|
//! by $1/(q+1)$ of this excess space, where $q$ is the number of times the
//! leader box will be replicated. Slight inaccuracies in the division might
//! accumulate; half of this rounding error is placed at each end of the leaders.
//!
//! @<Let |cur_h| be the position of the first box, ...@>=
//! if subtype(p)=a_leaders then
//!   begin save_h:=cur_h;
//!   cur_h:=left_edge+leader_wd*((cur_h-left_edge)@!div leader_wd);
//!   if cur_h<save_h then cur_h:=cur_h+leader_wd;
//!   end
//! else  begin lq:=rule_wd div leader_wd; {the number of box copies}
//!   lr:=rule_wd mod leader_wd; {the remaining space}
//!   if subtype(p)=c_leaders then cur_h:=cur_h+(lr div 2)
//!   else  begin lx:=lr div (lq+1);
//!     cur_h:=cur_h+((lr-(lq-1)*lx) div 2);
//!     end;
//!   end
//!
//! @ The `\\{synch}' operations here are intended to decrease the number of
//! bytes needed to specify horizontal and vertical motion in the \.{DVI} output.
//!
//! @<Output a leader box at |cur_h|, ...@>=
//! begin cur_v:=base_line+shift_amount(leader_box); synch_v; save_v:=dvi_v;@/
//! synch_h; save_h:=dvi_h; temp_ptr:=leader_box;
//! outer_doing_leaders:=doing_leaders; doing_leaders:=true;
//! if type(leader_box)=vlist_node then vlist_out@+else hlist_out;
//! doing_leaders:=outer_doing_leaders;
//! dvi_v:=save_v; dvi_h:=save_h; cur_v:=base_line;
//! cur_h:=save_h+leader_wd+lx;
//! end
//!
