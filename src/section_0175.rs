//! @ @<Print a short indication of the contents of node |p|@>=
//! case type(p) of
//! hlist_node,vlist_node,ins_node,whatsit_node,mark_node,adjust_node,
//!   unset_node: print("[]");
//! rule_node: print_char("|");
//! glue_node: if glue_ptr(p)<>zero_glue then print_char(" ");
//! math_node: print_char("$");
//! ligature_node: short_display(lig_ptr(p));
//! disc_node: begin short_display(pre_break(p));
//!   short_display(post_break(p));@/
//!   n:=replace_count(p);
//!   while n>0 do
//!     begin if link(p)<>null then p:=link(p);
//!     decr(n);
//!     end;
//!   end;
//! othercases do_nothing
//! endcases
//!
