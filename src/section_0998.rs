//! @ @<Link node |p| into the current page and |goto done|@>=
//! link(page_tail):=p; page_tail:=p;
//! link(contrib_head):=link(p); link(p):=null; goto done
//!
