//! @ @<Display rule |p|@>=
//! begin print_esc("rule("); print_rule_dimen(height(p)); print_char("+");
//! print_rule_dimen(depth(p)); print(")x"); print_rule_dimen(width(p));
//! end
//!
//! @ @<Display insertion |p|@>=
//! begin print_esc("insert"); print_int(qo(subtype(p)));
//! print(", natural size "); print_scaled(height(p));
//! print("; split("); print_spec(split_top_ptr(p),0);
//! print_char(","); print_scaled(depth(p));
//! print("); float cost "); print_int(float_cost(p));
//! node_list_display(ins_ptr(p)); {recursive call}
//! end
//!
