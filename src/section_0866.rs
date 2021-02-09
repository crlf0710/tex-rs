//! @ Here is the main switch in the |line_break| routine, where legal breaks
//! are determined. As we move through the hlist, we need to keep the |active_width|
//! array up to date, so that the badness of individual lines is readily calculated
//! by |try_break|. It is convenient to use the short name |act_width| for
//! the component of active width that represents real width as opposed to glue.
//
// @d act_width==active_width[1] {length from first active node to current node}
// @d kern_break==begin if not is_char_node(link(cur_p)) and auto_breaking then
//     if type(link(cur_p))=glue_node then try_break(0,unhyphenated);
//   act_width:=act_width+width(cur_p);
//   end
//
// @<Call |try_break| if |cur_p| is a legal breakpoint...@>=
// begin if is_char_node(cur_p) then
//   @<Advance \(c)|cur_p| to the node following the present
//     string of characters@>;
// case type(cur_p) of
// hlist_node,vlist_node,rule_node: act_width:=act_width+width(cur_p);
// whatsit_node: @<Advance \(p)past a whatsit node in the \(l)|line_break| loop@>;
// glue_node: begin @<If node |cur_p| is a legal breakpoint, call |try_break|;
//   then update the active widths by including the glue in |glue_ptr(cur_p)|@>;
//   if second_pass and auto_breaking then
//     @<Try to hyphenate the following word@>;
//   end;
// kern_node: if subtype(cur_p)=explicit then kern_break
//   else act_width:=act_width+width(cur_p);
// ligature_node: begin f:=font(lig_char(cur_p));
//   act_width:=act_width+char_width(f)(char_info(f)(character(lig_char(cur_p))));
//   end;
// disc_node: @<Try to break after a discretionary fragment, then |goto done5|@>;
// math_node: begin auto_breaking:=(subtype(cur_p)=after); kern_break;
//   end;
// penalty_node: try_break(penalty(cur_p),unhyphenated);
// mark_node,ins_node,adjust_node: do_nothing;
// othercases confusion("paragraph")
// @:this can't happen paragraph}{\quad paragraph@>
// endcases;@/
// prev_p:=cur_p; cur_p:=link(cur_p);
// done5:end
//