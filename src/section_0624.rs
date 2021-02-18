//! @ @<Output a rule in an hlist@>=
//! if is_running(rule_ht) then rule_ht:=height(this_box);
//! if is_running(rule_dp) then rule_dp:=depth(this_box);
//! rule_ht:=rule_ht+rule_dp; {this is the rule thickness}
//! if (rule_ht>0)and(rule_wd>0) then {we don't output empty rules}
//!   begin synch_h; cur_v:=base_line+rule_dp; synch_v;
//!   dvi_out(set_rule); dvi_four(rule_ht); dvi_four(rule_wd);
//!   cur_v:=base_line; dvi_h:=dvi_h+rule_wd;
//!   end
//!
