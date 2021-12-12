//! @ @d choose_mlist(#)==begin p:=#(q); #(q):=null;@+end
//!
//! @<Change this node to a style node...@>=
//! begin case cur_style div 2 of
//! 0: choose_mlist(display_mlist); {|display_style=0|}
//! 1: choose_mlist(text_mlist); {|text_style=2|}
//! 2: choose_mlist(script_mlist); {|script_style=4|}
//! 3: choose_mlist(script_script_mlist); {|script_script_style=6|}
//! end; {there are no other cases}
//! flush_node_list(display_mlist(q));
//! flush_node_list(text_mlist(q));
//! flush_node_list(script_mlist(q));
//! flush_node_list(script_script_mlist(q));@/
//! type(q):=style_node; subtype(q):=cur_style; width(q):=0; depth(q):=0;
//! if p<>null then
//!   begin z:=link(q); link(q):=p;
//!   while link(p)<>null do p:=link(p);
//!   link(p):=z;
//!   end;
//! goto done_with_node;
//! end
//!
