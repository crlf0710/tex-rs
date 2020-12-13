//! @ Here's a similar procedure that returns a pointer to a rule node. This
//! routine is called just after \TeX\ has seen \.{\\hrule} or \.{\\vrule};
//! therefore |cur_cmd| will be either |hrule| or |vrule|. The idea is to store
//! the default rule dimensions in the node, then to override them if
//! `\.{height}' or `\.{width}' or `\.{depth}' specifications are
//! found (in any order).
//!
//! @d default_rule=26214 {0.4\thinspace pt}
//!
//! @p function scan_rule_spec:pointer;
//! label reswitch;
//! var q:pointer; {the rule node being created}
//! begin q:=new_rule; {|width|, |depth|, and |height| all equal |null_flag| now}
//! if cur_cmd=vrule then width(q):=default_rule
//! else  begin height(q):=default_rule; depth(q):=0;
//!   end;
//! reswitch: if scan_keyword("width") then
//! @.width@>
//!   begin scan_normal_dimen; width(q):=cur_val; goto reswitch;
//!   end;
//! if scan_keyword("height") then
//! @.height@>
//!   begin scan_normal_dimen; height(q):=cur_val; goto reswitch;
//!   end;
//! if scan_keyword("depth") then
//! @.depth@>
//!   begin scan_normal_dimen; depth(q):=cur_val; goto reswitch;
//!   end;
//! scan_rule_spec:=q;
//! end;
//!
