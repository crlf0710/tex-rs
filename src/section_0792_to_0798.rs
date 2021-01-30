//! @ @<If the preamble list has been traversed, check that the row has ended@>=
//! if (p=null)and(extra_info(cur_align)<cr_code) then
//!  if cur_loop<>null then @<Lengthen the preamble periodically@>
//!  else  begin print_err("Extra alignment tab has been changed to ");
//! @.Extra alignment tab...@>
//!   print_esc("cr");
//!   help3("You have given more \span or & marks than there were")@/
//!   ("in the preamble to the \halign or \valign now in progress.")@/
//!   ("So I'll assume that you meant to type \cr instead.");
//!   extra_info(cur_align):=cr_code; error;
//!   end
//!
//! @ @<Lengthen the preamble...@>=
//! begin link(q):=new_null_box; p:=link(q); {a new alignrecord}
//! info(p):=end_span; width(p):=null_flag; cur_loop:=link(cur_loop);
//! @<Copy the templates from node |cur_loop| into node |p|@>;
//! cur_loop:=link(cur_loop);
//! link(p):=new_glue(glue_ptr(cur_loop));
//! end
//!
//! @ @<Copy the templates from node |cur_loop| into node |p|@>=
//! q:=hold_head; r:=u_part(cur_loop);
//! while r<>null do
//!   begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
//!   end;
//! link(q):=null; u_part(p):=link(hold_head);
//! q:=hold_head; r:=v_part(cur_loop);
//! while r<>null do
//!   begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
//!   end;
//! link(q):=null; v_part(p):=link(hold_head)
//!
//! @ @<Copy the tabskip glue...@>=
//! tail_append(new_glue(glue_ptr(link(cur_align))));
//! subtype(tail):=tab_skip_code+1
//!
//! @ @<Package an unset...@>=
//! begin if mode=-hmode then
//!   begin adjust_tail:=cur_tail; u:=hpack(link(head),natural); w:=width(u);
//!   cur_tail:=adjust_tail; adjust_tail:=null;
//!   end
//! else  begin u:=vpackage(link(head),natural,0); w:=height(u);
//!   end;
//! n:=min_quarterword; {this represents a span count of 1}
//! if cur_span<>cur_align then @<Update width entry for spanned columns@>
//! else if w>width(cur_align) then width(cur_align):=w;
//! type(u):=unset_node; span_count(u):=n;@/
//! @<Determine the stretch order@>;
//! glue_order(u):=o; glue_stretch(u):=total_stretch[o];@/
//! @<Determine the shrink order@>;
//! glue_sign(u):=o; glue_shrink(u):=total_shrink[o];@/
//! pop_nest; link(tail):=u; tail:=u;
//! end
//!
//! @ A span node is a 2-word record containing |width|, |info|, and |link|
//! fields. The |link| field is not really a link, it indicates the number of
//! spanned columns; the |info| field points to a span node for the same
//! starting column, having a greater extent of spanning, or to |end_span|,
//! which has the largest possible |link| field; the |width| field holds the
//! largest natural width corresponding to a particular set of spanned columns.
//!
//! A list of the maximum widths so far, for spanned columns starting at a
//! given column, begins with the |info| field of the alignrecord for that
//! column.
//!
//! @d span_node_size=2 {number of |mem| words for a span node}
//!
//! @<Initialize the special list heads...@>=
//! link(end_span):=max_quarterword+1; info(end_span):=null;
//!
//! @ @<Update width entry for spanned columns@>=
//! begin q:=cur_span;
//! repeat incr(n); q:=link(link(q));
//! until q=cur_align;
//! if n>max_quarterword then confusion("256 spans"); {this can happen, but won't}
//! @^system dependencies@>
//! @:this can't happen 256 spans}{\quad 256 spans@>
//! q:=cur_span; while link(info(q))<n do q:=info(q);
//! if link(info(q))>n then
//!   begin s:=get_node(span_node_size); info(s):=info(q); link(s):=n;
//!   info(q):=s; width(s):=w;
//!   end
//! else if width(info(q))<w then width(info(q)):=w;
//! end
//!
