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
