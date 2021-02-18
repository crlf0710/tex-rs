//! @ @<Output a rule in a vlist...@>=
//! if is_running(rule_wd) then rule_wd:=width(this_box);
//! rule_ht:=rule_ht+rule_dp; {this is the rule thickness}
//! cur_v:=cur_v+rule_ht;
//! if (rule_ht>0)and(rule_wd>0) then {we don't output empty rules}
//!   begin synch_h; synch_v;
//!   dvi_out(put_rule); dvi_four(rule_ht); dvi_four(rule_wd);
//!   end;
//! goto next_p
//!
