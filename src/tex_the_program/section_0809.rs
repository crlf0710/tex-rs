//! @ @<Append tabskip glue and an empty box to list |u|...@>=
//! s:=link(s); v:=glue_ptr(s); link(u):=new_glue(v); u:=link(u);
//! subtype(u):=tab_skip_code+1; t:=t+width(v);
//! if glue_sign(p)=stretching then
//!   begin if stretch_order(v)=glue_order(p) then
//!     t:=t+round(float(glue_set(p))*stretch(v));
//! @^real multiplication@>
//!   end
//! else if glue_sign(p)=shrinking then
//!   begin if shrink_order(v)=glue_order(p) then
//!     t:=t-round(float(glue_set(p))*shrink(v));
//!   end;
//! s:=link(s); link(u):=new_null_box; u:=link(u); t:=t+width(s);
//! if mode=-vmode then width(u):=width(s)@+else
//!   begin type(u):=vlist_node; height(u):=width(s);
//!   end
//!
