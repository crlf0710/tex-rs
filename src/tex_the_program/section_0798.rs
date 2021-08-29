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
