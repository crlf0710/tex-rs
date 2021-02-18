//! @ @<Output leaders in a vlist...@>=
//! begin leader_box:=leader_ptr(p);
//! if type(leader_box)=rule_node then
//!   begin rule_wd:=width(leader_box); rule_dp:=0;
//!   goto fin_rule;
//!   end;
//! leader_ht:=height(leader_box)+depth(leader_box);
//! if (leader_ht>0)and(rule_ht>0) then
//!   begin rule_ht:=rule_ht+10; {compensate for floating-point rounding}
//!   edge:=cur_v+rule_ht; lx:=0;
//!   @<Let |cur_v| be the position of the first box, and set |leader_ht+lx|
//!     to the spacing between corresponding parts of boxes@>;
//!   while cur_v+leader_ht<=edge do
//!     @<Output a leader box at |cur_v|,
//!       then advance |cur_v| by |leader_ht+lx|@>;
//!   cur_v:=edge-10; goto next_p;
//!   end;
//! end
//!
//! @ @<Let |cur_v| be the position of the first box, ...@>=
//! if subtype(p)=a_leaders then
//!   begin save_v:=cur_v;
//!   cur_v:=top_edge+leader_ht*((cur_v-top_edge)@!div leader_ht);
//!   if cur_v<save_v then cur_v:=cur_v+leader_ht;
//!   end
//! else  begin lq:=rule_ht div leader_ht; {the number of box copies}
//!   lr:=rule_ht mod leader_ht; {the remaining space}
//!   if subtype(p)=c_leaders then cur_v:=cur_v+(lr div 2)
//!   else  begin lx:=lr div (lq+1);
//!     cur_v:=cur_v+((lr-(lq-1)*lx) div 2);
//!     end;
//!   end
//!
//! @ When we reach this part of the program, |cur_v| indicates the top of a
//! leader box, not its baseline.
//!
//! @<Output a leader box at |cur_v|, ...@>=
//! begin cur_h:=left_edge+shift_amount(leader_box); synch_h; save_h:=dvi_h;@/
//! cur_v:=cur_v+height(leader_box); synch_v; save_v:=dvi_v;
//! temp_ptr:=leader_box;
//! outer_doing_leaders:=doing_leaders; doing_leaders:=true;
//! if type(leader_box)=vlist_node then vlist_out@+else hlist_out;
//! doing_leaders:=outer_doing_leaders;
//! dvi_v:=save_v; dvi_h:=save_h; cur_h:=left_edge;
//! cur_v:=save_v-height(leader_box)+leader_ht+lx;
//! end
//!
