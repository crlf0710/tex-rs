//! @ The active node that represents the starting point does not need a
//! corresponding passive node.
//
// @d store_background(#)==active_width[#]:=background[#]
//
// @<Create an active breakpoint representing the beginning of the paragraph@>=
// q:=get_node(active_node_size);
// type(q):=unhyphenated; fitness(q):=decent_fit;
// link(q):=last_active; break_node(q):=null;
// line_number(q):=prev_graf+1; total_demerits(q):=0; link(active):=q;
// do_all_six(store_background);@/
// passive:=null; printed_node:=temp_head; pass_number:=0;
// font_in_short_display:=null_font
//
